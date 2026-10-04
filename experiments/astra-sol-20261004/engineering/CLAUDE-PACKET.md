# Engineering packet for Claude, from Jeb / Astra–Sol research team

Jason authorized this transfer to the existing Building an AI in Excel conversation.
Research snapshot 2026-10-04. Local code and conditional proof artifacts below,
with relative paths so you can extract them. No secret, credential or private game data.
Every experiment, build/benchmark and Lean run must use an enforced <=300-second
process-group timeout; included watchdog allows <=295 seconds. No production deployment.

Pinned phone WASM: https://raw.githubusercontent.com/jasonyandell/plunge/a0d9fa806166b0e63fe016bb49d93f91f47b1af8/src/ai/phone/walt-player.wasm
SHA256: 40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119
Rust source: https://github.com/jasonyandell/texas-42/tree/cb1ef3b23072e4c268f31f625f2b61d5facc1929
Use it only after the exact hash matches; we did not check a currently served phone release.
The Python prototype is an explanatory finite-world Dice compiler, not a new Walt player.
The arena expects the exact WASM at experiments/astra-sol-20261004/reference/production-phone/walt-player.wasm.
The reference rules source is included at its original experiments/partnership/rules.py path.
Phase3 panel needs recorded phase2 fixtures; small standalone fixtures are attached below.
You can choose implementation details. Return a concrete patch/prototype plus bounded
commands and honest equality/cost evidence, with any production-baseline access blocker explicit.


## File: experiments/astra-sol-20261004/PROVENANCE.json

````
{
  "schema": "walt-astra-sol-research-v1",
  "date": "2026-10-04",
  "workspace": "/Users/jason/Documents/Codex/2026-10-04/task/research-worktree",
  "source_identity": {
    "production_commit": "a0d9fa806166b0e63fe016bb49d93f91f47b1af8",
    "source_commit": "cb1ef3b23072e4c268f31f625f2b61d5facc1929",
    "source_files": 202,
    "source_sha256": "5bedf14af0e5a5ef354acca4765f67843469d074d163e8e4786631aa23436e32",
    "wasm_sha256": "40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119",
    "source_inputs_match": true,
    "production_git_blobs_match": true,
    "platform": "macOS-26.5.1-arm64-arm-64bit",
    "machine": "arm64"
  },
  "initial_cpu_commit": "3cf2536dcbd6636bfff8ab814858ee00e8dc8b5e",
  "local_plunge_commit": "aff966d65028e3a7813afdb6111562d3272f2556",
  "registered_texas42_commit": "9d6a5a2e12958183d49d0cd95c8617c4ef7a12e4",
  "generator_commit": "06bed7532de2015412e4808dae24a5cf6d95d38f",
  "team": {
    "lead": "Astra-designated delegated task; runtime identity not independently attested",
    "collaborator": "gpt-6.1-sol explicitly requested and successfully launched via collaboration.spawn_agent",
    "reasoning": "high",
    "independent_context": true,
    "mutual_review": true
  },
  "toolchain": {
    "lean": "Lean (version 4.33.0-rc1, arm64-apple-darwin24.6.0, commit 62eed1db4d67327ec8120be05f1a1b0847d74561, Release)",
    "node": "v26.0.0"
  },
  "untouched_sources": true,
  "merged": false,
  "deployed": false,
  "baseline_scope": "exact supplied production WASM; straight bid-30 play with specified public seed; Mac Node host",
  "remote_issue": "https://github.com/jasonyandell/texas-42/issues/99"
}

````

## File: experiments/astra-sol-20261004/CLAIMS.md

````
# Numbered claim ledger for independent adversarial review

Status is deliberately scoped: accepted means a proved conditional statement,
reproduced finite observation, or checked source fact. It does not make every
application of that statement true. Contradicted claims are preserved in
[CONTRADICTED-FINDINGS.md](CONTRADICTED-FINDINGS.md). Unknown is not false.

| ID | Status | Falsifiable statement and primary evidence |
|---|---|---|
| C01 | Accepted identity | Retained phone WASM and release metadata match supplied Plunge a0d9fa8 and Rust cb1ef3b; SHA256 40d7a1ee…9119. `PROVENANCE.json`, `results/production-identity/`, `tools/verify_identity.py`. A different currently served release is outside scope. |
| C02 | Accepted code characterization | A completed finite response aggregates original scenarios at shared focal information states; it models specified nonfocal policies, not arbitrary adversaries or partner coordination. Phone defaults include staged complete fallbacks, voidless inner belief and optional heuristic partner review. `REPORT.md`, pinned Rust and `reference/production-phone/`. |
| C03 | Accepted conditional theorem | Uniform weighted payoff-disagreement bounds transport to maxima over the same lawful policy family and preserve sufficiently separated canonical actions. `math/lead/Disagreement.lean`; 5832 coupling / 11025 transport checks. |
| C04 | Accepted counterexample | Arbitrarily small inner regret can change an outer actor's conditional success from 1 to 0 when their beliefs differ. `math/sol/ArgmaxAmplification.lean`. It does not refute value continuity under one unchanged belief. |
| C05 | Accepted conditional theorem | Completed first-disagreement certificate expanding all legal focal actions proves equal trace/payoff for every lawful focal policy on unflagged scenarios. `phase2/math/sol/FirstDivergence.lean`; 8192 comparisons and 65 canonical certificates. |
| C06 | Accepted finite negative measurement | Current cold certificate+fresh fallback is slower on all four retained native panels (1.691×, 2.124×, 3.062×, 1.868×); 1/84 certificates on main panel. `phase2/results/cost-*`, independently audited. Cache-reusing fallback is not tested. |
| C07 | Accepted finite semantics | On 15 retained late roots, 40 worlds and 4 tapes, universal/adaptive compiled lawful-Dice trees match all 780 recursive action vectors; 192 lawful policies/tape independently enumerated on a smaller case. `phase3/results/panel-a`, `phase3/math/sol/{panel_audit,policy_audit}.py`. |
| C08 | Accepted conditional theorem | A topologically indexed graph's bottom-up SUM/MAX/MIN fold equals its recursive semantics. `phase3/math/sol/CompiledDAG.lean`. No proof of game compiler correctness, array access cost, graph-size bound or production speed is claimed. |
| C09 | Accepted finite cost/coverage observation | 533882/592717 (90.074%) universal coordinates unused across four tapes; adaptive cost is 0.632× reference for 13-payoff reuse, 2.992× for one bid in added panel; universal one-bid 20.552×. `phase3/results/panel-coverage`, independent `coverage-audit-run`. No population timing inference. |
| C10 | Accepted counterexample | Outer focal-conditioned world support may omit positive-mass worlds in the modeled actor's belief and reverse its optimal response. `phase3/math/sol/InnerFiber.lean`. Abstract finite example, not a Texas42 fixture or impossibility of expanded supports. |
| C11 | Accepted recovered finite example | Fable rational endgame under its supplied prior and future Dice gives 515/2016 vs 449/2016 bid-made probabilities; defender prefers latter. `phase2/results/fable-endgame-reproduction`. No exact-production-field claim. |
| C12 | Accepted source distinction | Original tiny-net uniform rollout teacher is lawful; creation source has row-level validation and a fused Dice comparator. It is neither issue100 nor compiled-Scheme student. `phase2/incoming/rollout_net.py`, packet provenance, Sol phase3 review. Later variants remain unverified. |
| C13 | Accepted source reuse opportunity | Actual Scheme compiles relational queries and composes exact belief updates with supplied likelihoods; current `scheme_worlds` is a world-membership batch tool. Compact step/game compilation is absent. `walt/scheme/{README,DYNAMICS,COMPOSITION}.md`, pinned Rust. |
| C14 | Accepted scoped baseline | Exact-phone arena reproduces 32 games/896 moves plus 8-game self-control with strict information requests, rotations/role swaps and independent audits. Four-cluster pilot interval [-1,1] establishes no strength ordering. `REPORT.md`, `results/production-panel`, `math/sol/panel-audit-run`. |
| C15 | Open conjecture | Compact native/layered support operations or valid cross-request reuse can make fixed-bid latency substantially cheaper. Needs full-build cost and exact-vector parity on unseen fixtures before paired phone games. |
| C16 | Open conjecture | Complete-key lower-rung caches, expanded belief supports and batched lookups can make useful higher k affordable. Need unique-key growth, support closure, hit rates, depth and total work measured; no linear-in-k theorem. |
| C17 | Open conjecture | Paired all-action/advantage distillation with raw information inputs can yield useful cheap modeled policies. Needs lawful teacher, held-out games/hands, regret/uncertainty and training amortization; no substantial training run here. |
| C18 | Open investigation | Prefix-likelihood bug needs a concrete Texas42 fixture; dummy-bidder replay needs an executable history; fixed-width history overflow needs an actual collision before claiming observed failure. Static/generic evidence is recorded without pretending those fixtures exist. |

Counterexample targets for the fresh team: duplicate scenario multiplicities;
different tapes on identical physical worlds; arbitrary partial tricks; equal
value ties; observer-dependent fields; refusal propagation; policy-action
likelihoods; reconvergent graph masks; field-query key purity; changes of focal
hand; and honest end-to-end cost on one fixed bid. A counterexample should cite
the exact ID and violated assumptions, and be preserved alongside the claim.

````

## File: experiments/astra-sol-20261004/phase2/REPORT.md

````
# Phase 2: safe substitution, actual cost, and recovered Fable work

2026-10-04. Exploratory research. The useful mathematical result is a sufficient
certificate for safely replacing one frozen modeled field with another. Its
current implementation is slower than solving the target on the retained
panels. The recovered Fable material supplies a useful exact-tail example, but
does not supply a verified production speed or strength improvement.

## Completed certificate and costs

For each root action and original scenario, traverse every legal focal branch
under the cheap field. At each nonfocal node compare completed cheap and target
field actions. Flag the entire scenario on the first difference. The absence
of a flag guarantees identical traces and payoffs for **every lawful focal
continuation**, because the certificate explores a superset of their paths.
World-specific branching is used only to bound an event; it does not evaluate
a world-specific optimal policy. Refusal/incomplete work never certifies.

If the flagged mass for action a is epsilon_a, then every lawful policy's
payoff changes by at most epsilon_a, and so does its attained maximum. Thus
old value minus epsilon is a lower bound and old value plus epsilon is an
upper bound. A candidate survives canonical least-tile selection only when
every lower-index competitor has strictly smaller upper bound and every
higher-index competitor has weakly smaller upper bound. These are sufficient,
potentially loose conditions on the **same weighted bundle and policy family**.

`math/sol/FirstDivergence.lean` proves the all-policy coupling premise missing
from phase 1. It formalizes arbitrary finite deterministic transitions,
observation-only focal policies, and a certificate expanding all legal focal
actions. Four theorems prove full trace/payoff equality, payoff equality,
contrapositive, and unflagged-world coupling. `math/sol/certificate_witness.py`
checks 256 field pairs, 16 lawful policies, weighted duplicates and refusal:
8,192 root/policy comparisons, 65 canonical certificates, zero false positives.

`probe/src/main.rs` is a separate Rust crate using the pinned Walt library;
the production code is unchanged. `run_cost.py` uses source deal 680000+i,
nine declarations, bidder i mod 4, random-legal complete-trick prefixes and a
40-world outer bundle. Eligibility (unsettled contract and a free choice) is
decided before probing. Input validity is established by the fixture audit;
the probe is not a hardened public request adapter.

| Frozen field comparison | Eligible | Certified | Cold certificate + fallback / fresh target |
|---|---:|---:|---:|
| L0 inner worlds 1 → 8 | 84 | 1 | 487000 / 288052 us = 1.691× |
| L0 inner worlds 4 → 8 | 84 | 1 | 609173 / 286855 us = 2.124× |
| Equal L0 field 8 → 8 control | 25 | 25 | 258832 / 84520 us = 3.062× |
| Modeled L1, n1=2, inner 1 → 8 | 25 | 3 | 1246378 / 667401 us = 1.868× |

Cold cost charges cheap solve and certificate on every input, plus a fresh
target solve on each rejected input. It does not reuse target-query cache on
fallback. Common sampling is omitted on both sides. Even the single accepted
1→8 input costs 115 us versus target 63 us. These are native kernel timings on
random-legal conditional positions, not phone-device latency or game strength.
Sol independently regenerated fixtures, checked support and every interval,
recomputed accounting and replayed accepted/control cases. Full details and
logs are in [Sol's report](math/sol/report.md).

## Recovered sources and scope

All 66 user/assistant message bodies in the authorized Claude conversation
[Building an AI in Excel](https://claude.ai/chat/58a73a5e-c5d2-4355-84a6-9f8234207aad)
were read. Collapsed tool transcripts were not exhaustively expanded; the
relevant original neural source was recovered from message 22. No message was
sent. This is a read/download provenance statement, not replication of all
historical experiments.

* `incoming/walt-sense.zip`: 39,874 bytes, SHA256
  `df5fa7294cf41f334233aa4a0e864384ad18d555bf08c2819b6a8ed2eb49f326`.
  Twelve files extracted under `incoming/walt-sense/walt-sense/` after path,
  symlink and size checks. This packet itself has no neural training source.
* `incoming/rollout_net.py`: original creation version, 9,019 bytes, SHA256
  `8b1d2bc68e9d2299031ce8393741bd7bdff9932dcd04dcb73e03217887710508`.
  It has top-level training code and was **read, not imported or executed**.
* `incoming/drive/`: all nine current files fetched from the supplied
  [Drive folder](https://drive.google.com/drive/folders/1-0rjkAYyIWZbb-Pg6cpqEn80T2wwNLMb),
  including HANDOFF.md and TINY-MODEL.md. Its provenance records original file
  IDs, URLs, timestamps and byte hashes. Seven `.diff` files preserve changes
  from the ZIP. These versions must not be silently interchanged.
* `reference/simple_walt/` is issue-100 branch commit
  `d45b3998120999552e324a8b60d58f1fefba7f8f`: a Python memo replay experiment,
  **not** the user's tiny neural experiment. The older compiled Scheme student
  campaign is a third, separate lineage.

## Accepted results, errors and unresolved claims in Fable material

The standalone rational `endgame.py` was read and reproduced under a 30-second
watchdog in 1.210 seconds. It enumerates 560 remaining deals and all weighted
future Dice continuations, keeping the defender's choice shared by public
history. Bid-made probability is 515/2016 after 6-3 and 449/2016 after 6-0;
the defending seat therefore prefers 6-0. Information-tree node counts are
48,795 and 48,709. This is exact for the supplied uniform root fiber and uniform
future other-seat policies; it does not infer action-likelihood posterior
weights for the prior observed history or model the production Walt field.
Receipt: `results/fable-endgame-reproduction/`.

The C `br_tape` and Python `exact_tape.py`/`engine42.walt_decide` optimize future
own actions separately per world before averaging. They are fused relaxations,
even when their executable root choice receives only legal own/public inputs.
The claimed 12–160× acceleration compares against a factorial-ordering
prototype. Pinned Rust Walt already branches recursively; this is not evidence
of speeding it up. C `exact_value` does share decisions and carry accumulated
chance weights, a useful model-specific building block (floating arithmetic).

The Python no-fusion stochastic fold groups histories but omits prefix reach
likelihoods at later focal maxima. Equal-prior worlds A/B/C reaching one
observation with likelihoods 1,1/3,1/3 and actions X winning B,C versus Y
winning A give an exact abstract reversal: unweighted X=2 beats Y=1, while
weighted X=2/3 loses to Y=1. A concrete Texas42 fixture for this bug remains
to be constructed. Frozen unit-weight tapes do not have this particular bug.

`engine42.py` replay initializes bidder=0 and calls it irrelevant, although
its game stops at bidder-dependent thresholds; that can truncate odd-bidder
histories. This is a static control-flow finding, not yet a replayed Texas42
counterexample. The fixed-width base-29 history representation may overflow
after 13 plies; no legal-history collision is established. The workbook builder
requires absent `scenes.json`. These scope distinctions are deliberate.

Historical match summaries report 376/720 native-L1 and 58/120 partner results.
They lack a pinned baseline hash and move-level receipts. Their harness defaults
partner off, changes opening budget/profile, lacks all four rotations, uses
naive hand-level uncertainty for paired data, and executes subprocesses without
hard timeouts. We did not run it or adopt its strength/speed inference. Current
Drive edits add batch seeds but do not repair those evidentiary gaps.

## Tiny-network diagnosis

The recovered uniform-rollout teacher is lawful: a simulator seeing the real
hidden world does not create fusion when the continuation policy is fixed and
information-admissible. Regression learns a conditional value under the
training occupancy distribution; transferring it to another posterior/policy
needs assumptions. `TeacherFusion.lean` separately proves an abstract example
where oracle-optimized labels produce the wrong lawful action, a distribution
shift reversal, and the usual 2-epsilon action-regret guarantee under explicit
action-value error bounds. It does **not** assert the uniform teacher is fused.

The creation script generates 14,000 uniform games for one own hand/trump,
attaches each game's final result to all its rows, and randomly splits rows.
Its validation is not a held-out-game/hand test. Its comparison invokes the
archive's fused `walt_decide`, not the exact production phone player. Later
reported paired-label/iteration variants, checkpoints, and complete raw logs
were not recovered, so their claims remain attributed, not reproduced.

The network has 56,577 parameters (226,308 bytes float32; 678,924 bytes for
parameters plus two Adam copies; these omit gradients/data). A 512×28×64 gather
alone is 3,670,016 bytes. This source is plain NumPy, not a GPU implementation.
Small weights do not establish cheap label generation or useful action ranking.
No substantial neural training, dependency installation, or paid work occurred.

Next viable tests should use a pinned lawful teacher, paired all-action vectors
and advantages, complete-game/hand splits, tie-aware regret, teacher uncertainty,
and explicit training/label amortization. Common random numbers reduce variance
only when their induced covariance helps. Fixed-budget approximate rungs can
cost linearly in their count without preserving exactness or fixed error;
policy iteration may cycle. Cycles do not imply absence of a mixed equilibrium.

## Reproduction

From the research worktree, use fresh output directories and the watchdog:

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 295 --output-dir /tmp/walt-build-2 -- cargo +1.95.0 build --offline --release --manifest-path experiments/astra-sol-20261004/phase2/probe/Cargo.toml
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-cost-log-2 -- python3 experiments/astra-sol-20261004/phase2/run_cost.py --out /tmp/walt-cost-2 --start 100 --count 144
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-certificate-proof-2 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/phase2/math/sol/FirstDivergence.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-teacher-proof-2 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/phase2/TeacherFusion.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-certificate-cases-2 -- python3 experiments/astra-sol-20261004/phase2/math/sol/certificate_witness.py
```

Use `--cheap 4`, `--cheap 8`, or `--level 1` for the follow-up cost variants;
the retained plans record exact ranges. Do not execute incoming training scripts
or the historical harness as a reproduction shortcut.

````

## File: experiments/astra-sol-20261004/phase3/REPORT.md

````
# Phase 3: tape coordinates, opposing dependencies and Scheme

2026-10-04. Exploratory Python prototype. All 780 retained finite action-value
vectors matched a grouped recursive reference, with an independent full replay
and an exhaustive lawful-policy check. This establishes a useful semantic
prototype. It does not establish faster phone Walt or linear exact higher k.

## The compiled object and two passes

`compiled_tape.py` fixes the focal own hand, complete public root history,
remaining sampled worlds and their multiplicities. Each private nature scenario
also has a frozen 64-bit draw per continuation ply. At nonfocal nodes the draw
selects one legal tile; the tape is not an input to the focal policy. A coordinate
is the complete chronological public tile history, with actor determined from
the root mechanics. The current implementation is a **tree** of grouped
histories, with no transposition merging.

The universal constructor retains every legal nonfocal edge for the supplied
physical worlds. The adaptive constructor retains only edges realized by the
fixed tapes. Both retain every focal action and play to completion. A forward
pass routes scenario-support masks through observed nonfocal actions. A reverse
pass assigns terminal success counts, SUMs over those public observations, and
MAXimizes one common focal choice after aggregation. This is a lawful finite
response to frozen Dice scenarios. It is not a response to Walt's lower-rung
policies, exact integration over all chance, or a calibrated game posterior.

The recursive control independently groups the same original scenarios and
uses safe contract-settled early termination. Values are integer successful
scenario counts in focal-team orientation. Root ties can use least tile.
Tape-specific graphs explicitly reject different tapes. No compact-state
sufficiency or unsafe history merging is assumed. The adapter is tested on
complete-trick roots; arbitrary partially played roots need their own adapter
validation. Compilation refuses at 100,000 nodes or 20 seconds; every panel
also runs under the process-group watchdog.

## Actual bounded measurements

Fifteen preselected eligible roots, plies 16 or 20, 40 worlds, four tapes and
13 bid thresholds 30–42 give 780 vectors. Both universal and adaptive compiled
values equal the recursive control on all of them. An additional bid-30
full-completion reference matches the early-stop reference. The panel selects
the first eligible source fixture for each available declaration/ply pair;
only 15 such pairs occur in the frozen source range. This is a small conditional
fixture panel, not an independent random sample of production requests.

| Retained workload | Original panel | Added coverage / single-bid panel |
|---|---:|---:|
| Universal compile + route + 13 reductions | 2.726655 s | 2.729114 s |
| Adaptive compile + route + 13 reductions | 0.570631 s | 0.572854 s |
| 13 independent recursive responses | 0.894150 s | 0.906986 s |
| Adaptive / reference, 13-payoff reuse | 0.638× | 0.632× |
| Adaptive compile + route + bid-30 reduction | not timed separately | 0.359957 s |
| Bid-30 recursive response | not timed separately | 0.120290 s |
| Adaptive / reference, one payoff | not established | 2.992× |

Universal one-payoff work still amortizes its build over four tapes; it costs
2.472145 s, about 20.55× the 0.120290 s reference. A fresh graph for each tape
would cost more. The 13-payoff benefit is genuine **within this prototype's
repeated-objective workload**, which ordinary one-contract play does not supply.
No uncertainty interval or generalized speed claim is inferred from these
single sequential timing observations; timing order was not randomized.

Universal trees total 592,717 coordinates and 674,101 support incidences.
Across four tapes per root only 58,835 distinct coordinates are used, leaving
533,882 (90.07%) unused. Active visits total 73,104. The separately reported
32.43× ratio is four-times-universal capacity / tape visits, **not** the union
unused fraction. Union active support incidences are 71,816; per-depth widths
are retained. Largest Python object census: 64,894,815 bytes, excluding full
process RSS and no inference about a compact native/GPU representation.
No compile refusals occurred; future refused-attempt time is explicitly charged.

`results/panel-a/` is original evidence, independently replayed by Sol in
14.911 seconds. `results/panel-coverage/` adds union coverage and single-bid
timing; its watchdog completed in 7.441 seconds. These two runs are not pooled
as a claimed statistically controlled performance study.

## Mathematical verification and independent challenge

Sol's `math/sol/CompiledDAG.lean` proves recursive and indexed bottom-up folds
equal on a finite graph whose node i references only children in `Fin i`.
The theorem permits DAG sharing, while this prototype uses a tree. Leaf mass,
SUM, MAX and MIN operations are explicit. Graph construction lawfulness,
actual array indexing, bounded-width arithmetic, compile complexity and hardware
speed are outside the theorem. The abstract store is a function, so the Lean
definition itself does not prove constant-time array lookups.

Exact two-world MAX/MIN counterexamples show that world-first optimization
changes semantics. Sol independently enumerated all 192 information-only focal
policies per tape on source case 126 with two scenarios and two tapes. Every
payoff optimum matched compiler and recursive vectors (tile 5/12/17: 1/0/1).
Each adaptive tree had 108 nodes versus universal 4,059. Wrong-tape reuse was
rejected. The six fold theorems and two inner-fiber theorems were independently
read and rerun by the lead; actual logs and axiom reports are retained.

## Why this does not yet flatten the best-response ladder

There are two distinct dependencies. Within a frozen field, forward support
flows through public actions, and reverse optimal values depend on future
children. Work can be parallel within independent scenarios/mechanical branches
or topological layers, but lawful focal maxima still couple scenarios. Across
rungs, a modeled actor's action requires a lower-rung response under **that
actor's information**, often with a different hidden-world fiber.

Outer worlds condition on the focal hand. An opponent does not know that hand;
its inner belief can contain different focal hands, absent from every outer
world. `InnerFiber.lean` proves a strict finite reversal: outer support has
hidden-bit counts 1:0, while an indistinguishing modeled actor's own belief has
counts 1:3. Restriction to the outer support flips its unique preferred guess.
This is an abstract coverage counterexample, not a concrete Texas42 deal or
an impossibility theorem for expanded-support compilation.

For a graph with explicit constant-time adjacency/value access and bounded
arithmetic, a fold costs O(nodes + edges). This says nothing linear in rung k
unless unique query/coordinate growth is independently controlled. Compilation
can materialize exponentially many focal histories or distinct lower-rung keys.
For reusable compile cost C, evaluation E and recursive cost R, m reuses help
only when m(R−E)>C. Reusing physical mechanics does not authorize reusing stale
posteriors, tapes, policy answers, multiplicities or omitted action branches.

## The actual project Scheme representation

Inspected at pinned source cb1ef3b: `walt/scheme/README.md`, `DYNAMICS.md`,
`COMPOSITION.md`, and Rust `scheme/{syntax,eval,dynamics,policy}.rs` plus
`bin/scheme_worlds.rs`. This is the project's relational Scheme/Fix language,
not the unrelated Scheme programming language.

`Fix::compile(&Registry)` validates an AST and captures immutable predicate
handles with versioned identity. It is useful ahead-of-time query specialization,
but is not an existing game-tree compiler. `scheme_worlds` compiles 1–64 queries
once and emits membership bitmasks for supplied opening-world rows. It does not
enumerate all positions, provide policy/outcome labels, or implement higher k.

`step_belief` already has the right exact composition order: legal filter,
multiply caller-provided observation likelihood, step worlds, merge successors
and report observation probability. Its documentation warns that physical-hand
marginals are insufficient when policy memory/tapes differ; that state must
be carried before merging. `anchor_back` is an analyst's hindsight preimage,
not information a past actor lawfully possessed. Compact Scheme step compilation
and its compression theorem are explicitly absent.

Donor composition retains a union of successful focal actions by public
history; complete donors preserve the specified fixed-sample Boolean optimum.
Partial donor coverage has no such guarantee, and changed action restrictions
invalidate related caches. These are concrete reuse components to investigate,
with proof obligations already stated by the project.

The prior generator commit 06bed753 parallelizes independent mandatory trials
while committing an ordered contiguous result prefix. Its reported ~4.4× tail
wall speedup used more CPU work and an unchanged frozen native player. It is
infrastructure evidence, not a phone search or higher-rung scaling result.

## Reproduce and challenge next

```sh
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 290 --output-dir /tmp/walt-tape-log-2 -- python3 experiments/astra-sol-20261004/phase3/run_panel.py --out /tmp/walt-tape-panel-2 --limit 18 --worlds 40 --tapes 4
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-tape-policy-2 -- python3 experiments/astra-sol-20261004/phase3/math/sol/policy_audit.py
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-fold-proof-2 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/phase3/math/sol/CompiledDAG.lean
python3 experiments/astra-sol-20261004/tools/run_capped.py --seconds 30 --output-dir /tmp/walt-fiber-proof-2 -- lean +leanprover/lean4:v4.33.0-rc1 experiments/astra-sol-20261004/phase3/math/sol/InnerFiber.lean
```

The fresh team should try to break this compiler on unseen own hands, weighted
duplicates, partial tricks and deeper roots before native acceleration; measure
fixed-contract preprocessing honestly; and instrument complete lower-rung
information keys/fiber coverage before asserting higher-k reuse. A native
support-bitset/layered array implementation, useful repeated-objective queries,
and compressed Scheme support remain viable conjectures. No training or
production changes are part of this checkpoint.

````

## File: experiments/astra-sol-20261004/phase3/compiled_tape.py

````
#!/usr/bin/env python3
"""Lawful sampled best response: fixed worlds, public-history coordinates.

Research prototype only. A tape is a private nature scenario, not information
available to the focal policy. A shared MAX follows aggregation over that
history's surviving scenarios. Universal coordinates retain every legal other-
seat edge; tape-specific coordinates retain only realized other-seat edges.
Both preserve full public history and play to completion for payoff reuse.
"""
from dataclasses import dataclass
from pathlib import Path
import random, sys, time
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT.parent/'partnership'))
from rules import legal_tiles, winner, trick_points
popcount=getattr(int,'bit_count',lambda x:bin(x).count('1'))

def bits(mask):
    while mask:
        one=mask & -mask;yield one.bit_length()-1;mask-=one

@dataclass(frozen=True)
class State:
    played:int
    leader:int
    trick:tuple
    points:tuple

@dataclass
class Node:
    key:int
    depth:int
    support:int
    actor:int
    focal:bool
    children:dict
    legal:dict
    score:object

class Kernel:
    def __init__(self,request,worlds,points,leader,trick=()):
        self.request=request;self.worlds=worlds;self.n=len(worlds)
        self.focal=request['seat'];self.bidder=request['bidder'];self.decl=request['decl']
        self.root=State(sum(1<<t for t in request['plays'][1::2]),leader,tuple(trick),tuple(points))
        self.plies=28-popcount(self.root.played)
        assert self.n and 0<self.plies<=16
        known=sum(1<<t for t in request['hand']) & ~self.root.played
        for world in worlds:
            assert world[self.focal]==known and not (sum(world)&self.root.played)
            assert sum(world)==((1<<28)-1)^self.root.played
            union=0
            for hand in world:
                assert hand&union==0;union|=hand

    def moves(self,state,w):
        seat=(state.leader+len(state.trick))%4
        return legal_tiles(tuple(bits(self.worlds[w][seat]&~state.played)),state.trick,self.decl)

    def after(self,state,tile):
        seat=(state.leader+len(state.trick))%4;trick=state.trick+((seat,tile),)
        leader=state.leader;points=list(state.points)
        if len(trick)==4:
            leader=winner(trick,self.decl);points[leader%2]+=trick_points(trick);trick=()
        return State(state.played|(1<<tile),leader,trick,tuple(points))

    def success(self,score,bid):
        made=score[self.bidder%2]>=bid
        return made if self.focal%2==self.bidder%2 else not made

    def compile(self,tape=None,*,node_cap=100000,seconds=20):
        nodes=[];deadline=time.monotonic()+seconds
        def rec(state,support,key,depth):
            if len(nodes)>=node_cap or time.monotonic()>deadline:raise TimeoutError('structural compilation cap')
            i=len(nodes);actor=(state.leader+len(state.trick))%4
            node=Node(key,depth,support,actor,actor==self.focal,{}, {},None);nodes.append(node)
            if popcount(state.played)==28:
                assert sum(state.points)==42 and not state.trick
                node.score=state.points;return i
            groups={}
            for w in bits(support):
                legal=self.moves(state,w);node.legal[w]=tuple(legal)
                choices=legal if node.focal or tape is None else [pick(tape,w,depth,legal)]
                for tile in choices:groups[tile]=groups.get(tile,0)|(1<<w)
            if node.focal:assert len(set(node.legal.values()))==1
            for tile,mask in sorted(groups.items()):
                node.children[tile]=rec(self.after(state,tile),mask,key*29+tile+1,depth+1)
            return i
        rec(self.root,(1<<self.n)-1,0,0)
        assert len({n.key for n in nodes})==len(nodes)
        return Compiled(self,nodes,tape)

    def reference(self,tape,bid,early_stop=True):
        """Independent recursive control: split worlds first, share own MAX."""
        calls=0
        def rec(state,worlds,depth):
            nonlocal calls;calls+=1
            if early_stop and (state.points[self.bidder%2]>=bid or state.points[1-self.bidder%2]>42-bid):
                return int(self.success(state.points,bid))*len(worlds)
            if popcount(state.played)==28:return int(self.success(state.points,bid))*len(worlds)
            actor=(state.leader+len(state.trick))%4
            if actor==self.focal:
                legal=self.moves(state,worlds[0])
                assert all(self.moves(state,w)==legal for w in worlds)
                return max(rec(self.after(state,t),worlds,depth+1) for t in legal)
            groups={}
            for w in worlds:
                legal=self.moves(state,w);tile=pick(tape,w,depth,legal);groups.setdefault(tile,[]).append(w)
            return sum(rec(self.after(state,t),ws,depth+1) for t,ws in groups.items())
        assert (self.root.leader+len(self.root.trick))%4==self.focal
        values={t:rec(self.after(self.root,t),list(range(self.n)),1) for t in self.moves(self.root,0)}
        return values,calls

def pick(tape,w,depth,legal):
    # Exact function of the saved finite tape. No claim of exact integration
    # over continuous random numbers, posterior fibers, or all future draws.
    return legal[(tape[w][depth]*len(legal))>>64]

def make_tape(seed,worlds,plies):
    rng=random.Random(seed)
    return tuple(tuple(rng.getrandbits(64) for _ in range(plies)) for _ in range(worlds))

class Compiled:
    def __init__(self,kernel,nodes,tape):self.kernel=kernel;self.nodes=nodes;self.tape=tape

    def route(self,tape):
        """Forward lookup pass; returns only reached coordinates and masks."""
        if self.tape is not None:
            assert tape==self.tape,'Tape-specific coordinates must not be reused for a different tape'
            return [(i,n.support) for i,n in enumerate(self.nodes)]
        reached=[];todo=[(0,(1<<self.kernel.n)-1)]
        while todo:
            i,mask=todo.pop();node=self.nodes[i];reached.append((i,mask))
            assert mask and mask&~node.support==0
            if node.score is not None:continue
            if node.focal:
                for j in node.children.values():todo.append((j,mask))
            else:
                groups={}
                for w in bits(mask):
                    t=pick(tape,w,node.depth,node.legal[w]);groups[t]=groups.get(t,0)|(1<<w)
                for t,m in groups.items():todo.append((node.children[t],m))
        return reached

    def reduce(self,reached,bid):
        """Reverse lookup pass: aggregate scenarios before one shared MAX."""
        values={}
        for i,mask in reversed(reached):
            n=self.nodes[i]
            if n.score is not None:v=int(self.kernel.success(n.score,bid))*popcount(mask)
            elif n.focal:v=max(values[j] for j in n.children.values())
            else:v=sum(values.get(j,0) for j in n.children.values())
            values[i]=v
        return {tile:values[j] for tile,j in self.nodes[0].children.items()}

    def statistics(self):
        # Object-size census, not process RSS or a compact native format.
        seen=set()
        def size(x):
            if id(x) in seen:return 0
            seen.add(id(x));n=sys.getsizeof(x)
            if isinstance(x,dict):return n+sum(size(k)+size(v) for k,v in x.items())
            if isinstance(x,(list,tuple)):return n+sum(map(size,x))
            if isinstance(x,Node):return n+size(vars(x))
            return n
        return dict(nodes=len(self.nodes),edges=sum(len(n.children) for n in self.nodes),
            leaves=sum(n.score is not None for n in self.nodes),
            support_incidences=sum(popcount(n.support) for n in self.nodes),
            python_object_bytes=size(self.nodes),max_depth=max(n.depth for n in self.nodes))

````

## File: experiments/astra-sol-20261004/phase3/run_panel.py

````
#!/usr/bin/env python3
"""Bounded tape-coordinate experiment; launch with run_capped.py <=290s."""
import argparse, hashlib, json, sys, time
from pathlib import Path
from compiled_tape import Kernel,make_tape,ROOT,popcount

def timed(fn):
    t=time.perf_counter_ns();value=fn();return value,(time.perf_counter_ns()-t)/1000

def main():
    p=argparse.ArgumentParser();p.add_argument('--out',type=Path,required=True)
    p.add_argument('--limit',type=int,default=18);p.add_argument('--worlds',type=int,default=40)
    p.add_argument('--tapes',type=int,default=4);p.add_argument('--start',type=int,default=0)
    args=p.parse_args();assert 1<=args.worlds<=40 and 1<=args.tapes<=32
    args.out.mkdir(parents=True,exist_ok=False)
    source=ROOT/'phase2/results/cost-panel-a';plan=json.loads((source/'plan.json').read_text())
    fixtures=[];seen=set()
    for row in plan['rows']:
        key=(row['request']['decl'],row['ply'])
        if row['eligible'] and row['ply'] in (16,20) and key not in seen:
            seen.add(key);fixtures.append(row)
    fixtures=fixtures[args.start:args.start+args.limit]
    receipt_plan=dict(schema='walt-lawful-tape-coordinate-plan-v1',fixtures=fixtures,
        world_count=args.worlds,tape_seeds=list(range(790001,790001+args.tapes)),
        bids=list(range(30,43)),compilation_node_cap=100000,compilation_seconds=20,
        panel_seconds=270,source_panel_sha256=hashlib.sha256((source/'plan.json').read_bytes()).hexdigest(),
        python=sys.version,source_hashes={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (Path(__file__),Path(__file__).with_name('compiled_tape.py'))},
        semantics='Fixed 64-bit nature tapes and same sampled hidden worlds; lawful shared focal decisions. Not exact integration over hidden-world posterior or nature, and not production Walt field.')
    (args.out/'plan.json').write_text(json.dumps(receipt_plan,indent=2)+'\n')
    began=time.monotonic();receipts=[]
    for row in fixtures:
        if time.monotonic()-began>240:
            receipts.append(dict(id=row['id'],status='panel_deadline'));continue
        case_began=time.monotonic()
        request=row['request'];rotation=1 if request['bidder']%2==0 else 0
        stored=json.loads((source/f"case-{row['id']:04}.json").read_text())['result']['worlds'][:args.worlds]
        worlds=[[w[(seat+rotation)%4] for seat in range(4)] for w in stored]
        kernel=Kernel(request,worlds,row['points'],request['seat'])
        result=dict(id=row['id'],ply=row['ply'],decl=request['decl'],worlds=worlds,request=request,tapes=[])
        try:
            universal,compile_us=timed(lambda:kernel.compile())
            result.update(universal_compile_us=compile_us,universal_statistics=universal.statistics())
            used={};visits=0;support_visits=0
            for seed in receipt_plan['tape_seeds']:
                tape=make_tape(seed,kernel.n,kernel.plies)
                references,reference_us=timed(lambda:{b:kernel.reference(tape,b) for b in receipt_plan['bids']})
                active,route_us=timed(lambda:universal.route(tape))
                for i,mask in active:used[i]=used.get(i,0)|mask
                visits+=len(active);support_visits+=sum(popcount(mask) for _,mask in active)
                vectors,reduce_us=timed(lambda:{b:universal.reduce(active,b) for b in receipt_plan['bids']})
                adaptive,adaptive_compile_us=timed(lambda:kernel.compile(tape))
                adaptive_active,adaptive_route_us=timed(lambda:adaptive.route(tape))
                adaptive_vectors,adaptive_reduce_us=timed(lambda:{b:adaptive.reduce(adaptive_active,b) for b in receipt_plan['bids']})
                assert vectors==adaptive_vectors=={b:v[0] for b,v in references.items()}
                assert len(active)==len(adaptive.nodes)
                single_reference,single_reference_us=timed(lambda:kernel.reference(tape,30))
                single_universal,single_universal_us=timed(lambda:universal.reduce(active,30))
                single_adaptive,single_adaptive_us=timed(lambda:adaptive.reduce(adaptive_active,30))
                assert single_reference[0]==single_universal==single_adaptive==vectors[30]
                # Control the reference's early-stop optimization separately.
                full_reference,full_us=timed(lambda:kernel.reference(tape,30,False))
                assert full_reference[0]==vectors[30]
                record=dict(seed=seed,tape=tape,reference_13_bids_us=reference_us,
                    reference_calls={b:v[1] for b,v in references.items()},
                    universal_route_us=route_us,universal_reduce_13_bids_us=reduce_us,
                    adaptive_compile_us=adaptive_compile_us,adaptive_route_us=adaptive_route_us,
                    adaptive_reduce_13_bids_us=adaptive_reduce_us,adaptive_statistics=adaptive.statistics(),
                    reference_bid30_us=single_reference_us,universal_reduce_bid30_us=single_universal_us,
                    adaptive_reduce_bid30_us=single_adaptive_us,
                    full_reference_bid30_us=full_us,full_reference_bid30_calls=full_reference[1],
                    values=vectors,all_values_equal=True)
                result['tapes'].append(record)
            result['coverage']=dict(union_coordinates=len(used),union_support_incidences=sum(popcount(mask) for mask in used.values()),
                tape_coordinate_visits=visits,tape_support_visits=support_visits,
                unused_coordinates=len(universal.nodes)-len(used),
                depth_counts={d:dict(physical=sum(n.depth==d for n in universal.nodes),used=sum(universal.nodes[i].depth==d for i in used)) for d in range(kernel.plies+1)})
            result['status']='completed'
        except TimeoutError as e:result.update(status='compilation_refused',error=str(e))
        result['attempt_seconds']=time.monotonic()-case_began
        (args.out/f"case-{row['id']:04}.json").write_text(json.dumps(result,indent=2)+'\n');receipts.append(result)
    complete=[r for r in receipts if r['status']=='completed']
    pairs=[t for r in complete for t in r['tapes']]
    summary=dict(planned=len(fixtures),completed=len(complete),statuses={s:sum(r['status']==s for r in receipts) for s in {r['status'] for r in receipts}},
        source_cases=[r['id'] for r in complete],tapes=len(pairs),root_action_vectors_compared=len(pairs)*13,
        all_complete_vectors_equal=all(t['all_values_equal'] for t in pairs),elapsed_seconds=time.monotonic()-began,
        refused_attempt_seconds=sum(r.get('attempt_seconds',0) for r in receipts if r['status']!='completed'),
        universal_compile_us=sum(r['universal_compile_us'] for r in complete),
        universal_route_us=sum(t['universal_route_us'] for t in pairs),
        universal_reduce_13_bids_us=sum(t['universal_reduce_13_bids_us'] for t in pairs),
        reference_13_bids_us=sum(t['reference_13_bids_us'] for t in pairs),
        adaptive_compile_us=sum(t['adaptive_compile_us'] for t in pairs),
        adaptive_route_us=sum(t['adaptive_route_us'] for t in pairs),
        adaptive_reduce_13_bids_us=sum(t['adaptive_reduce_13_bids_us'] for t in pairs),
        reference_bid30_us=sum(t['reference_bid30_us'] for t in pairs),
        universal_reduce_bid30_us=sum(t['universal_reduce_bid30_us'] for t in pairs),
        adaptive_reduce_bid30_us=sum(t['adaptive_reduce_bid30_us'] for t in pairs),
        universal_nodes=sum(r['universal_statistics']['nodes'] for r in complete),
        union_used_coordinates=sum(r['coverage']['union_coordinates'] for r in complete),
        unused_coordinates=sum(r['coverage']['unused_coordinates'] for r in complete),
        union_used_support_incidences=sum(r['coverage']['union_support_incidences'] for r in complete),
        tape_coordinate_visits=sum(r['coverage']['tape_coordinate_visits'] for r in complete),
        universal_max_python_object_bytes=max((r['universal_statistics']['python_object_bytes'] for r in complete),default=0),
        note='Python research timing, each vector uses same frozen worlds and tape; no phone strength or speed claim. Includes compile costs explicitly; universal graph may hit node/time cap.')
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if __name__=='__main__':main()

````

## File: experiments/astra-sol-20261004/phase3/math/sol/CompiledDAG.lean

````
import Std

/- Abstract finite DAG evaluation, conditional on a correct lawful graph.
   Node i refers only to earlier nodes through Fin i. SUM is a nonfocal
   bucket fold; MAX/MIN must cover shared focal actions, not hidden worlds.
   Cache below is an abstract persistent indexed store, not a timing model. -/
namespace WaltResearch.Phase3.Sol

inductive Node (i : Nat) where
  | leaf (mass : Nat)
  | sum (children : List (Fin i))
  | maximum (children : List (Fin i))
  | minimum (ceiling : Nat) (children : List (Fin i))

def Node.fold {i : Nat} (node : Node i) (values : Fin i → Nat) : Nat :=
  match node with
  | .leaf mass => mass
  | .sum children => (children.map values).sum
  | .maximum children => (children.map values).foldl Nat.max 0
  | .minimum ceiling children => (children.map values).foldl Nat.min ceiling

theorem fold_congr {i : Nat} (node : Node i) (f g : Fin i → Nat)
    (same : ∀ j, f j = g j) : node.fold f = node.fold g := by
  have maps : ∀ xs : List (Fin i), xs.map f = xs.map g := by
    intro xs
    exact List.map_congr_left (fun j _ => same j)
  cases node <;> simp only [Node.fold, maps]

def recursive (graph : (i : Nat) → Node i) (i : Nat) : Nat :=
  (graph i).fold (fun j => recursive graph j.val)
termination_by i
decreasing_by exact j.isLt

def compiled (graph : (i : Nat) → Node i) : Nat → (Nat → Nat)
  | 0 => fun _ => 0
  | n + 1 =>
      let previous := compiled graph n
      let value := (graph n).fold (fun j => previous j.val)
      fun i => if i = n then value else previous i

theorem compiled_prefix_agrees (graph : (i : Nat) → Node i) (n : Nat) :
    ∀ i, i < n → compiled graph n i = recursive graph i := by
  induction n with
  | zero => intro i h; omega
  | succ n ih =>
      intro i hi
      by_cases eq : i = n
      · subst i
        simp only [compiled, ↓reduceIte]
        rw [recursive]
        exact fold_congr (graph n) _ _ (fun j => ih j.val j.isLt)
      · have lt : i < n := by omega
        simpa only [compiled, if_neg eq] using ih i lt

theorem compiled_root_equals_recursive (graph : (i : Nat) → Node i)
    (size : Nat) (root : Fin size) :
    compiled graph size root.val = recursive graph root.val :=
  compiled_prefix_agrees graph size root.val root.isLt

-- Frozen graph, arbitrary changing leaf scores: evaluation equivalence is
-- uniform in every graph. Changing tapes/fields may change the graph itself.
theorem two_evaluators_agree_for_every_graph :
    ∀ (graph : (i : Nat) → Node i) (size : Nat) (root : Fin size),
      compiled graph size root.val = recursive graph root.val := by
  intros
  exact compiled_root_equals_recursive _ _ _

-- Two equally weighted worlds share one focal observation. The two lawful
-- constant actions have payoff vectors [0,1] and [1,0]. Maximizing each world
-- before summing invents two different actions at that same observation.
theorem world_first_max_changes_semantics :
    Nat.max (0 + 1) (1 + 0) < Nat.max 0 1 + Nat.max 1 0 := by decide

theorem world_first_min_changes_semantics :
    Nat.min 0 1 + Nat.min 1 0 < Nat.min (0 + 1) (1 + 0) := by decide

#print axioms fold_congr
#print axioms compiled_prefix_agrees
#print axioms compiled_root_equals_recursive
#print axioms two_evaluators_agree_for_every_graph
#print axioms world_first_max_changes_semantics
#print axioms world_first_min_changes_semantics

end WaltResearch.Phase3.Sol

````

## File: experiments/astra-sol-20261004/phase3/math/sol/InnerFiber.lean

````
import Std

/- An outer focal player knows hidden bit x=false. A modeled opponent has
   one constant observation and does not know x. Its own belief contains four
   equally weighted worlds: one x=false, three x=true. Outer mechanical support
   conditioned on the focal information contains no x=true world. Optimizing
   the modeled actor over that outer support silently grants focal information.
   This is an abstract coverage obstacle, not a concrete Texas42 deal. -/
namespace WaltResearch.Phase3.Sol

def guessCount (falseMass trueMass : Nat) (guess : Bool) : Nat :=
  if guess then trueMass else falseMass

theorem outer_restriction_reverses_modeled_response :
    guessCount 1 0 true < guessCount 1 0 false ∧
    guessCount 1 3 false < guessCount 1 3 true := by decide

theorem outer_support_omits_inner_world :
    guessCount 1 0 true = 0 ∧ 0 < guessCount 1 3 true := by decide

#print axioms outer_restriction_reverses_modeled_response
#print axioms outer_support_omits_inner_world

end WaltResearch.Phase3.Sol

````

## File: experiments/astra-sol-20261004/phase3/math/sol/lean-compiled-dag-final-run/stdout.log

````
'WaltResearch.Phase3.Sol.fold_congr' depends on axioms: [propext, Quot.sound]
'WaltResearch.Phase3.Sol.compiled_prefix_agrees' depends on axioms: [propext, Quot.sound]
'WaltResearch.Phase3.Sol.compiled_root_equals_recursive' depends on axioms: [propext, Quot.sound]
'WaltResearch.Phase3.Sol.two_evaluators_agree_for_every_graph' depends on axioms: [propext, Quot.sound]
'WaltResearch.Phase3.Sol.world_first_max_changes_semantics' does not depend on any axioms
'WaltResearch.Phase3.Sol.world_first_min_changes_semantics' does not depend on any axioms

````

## File: experiments/astra-sol-20261004/phase3/math/sol/lean-inner-fiber-run/stdout.log

````
'WaltResearch.Phase3.Sol.outer_restriction_reverses_modeled_response' does not depend on any axioms
'WaltResearch.Phase3.Sol.outer_support_omits_inner_world' does not depend on any axioms

````

## File: experiments/astra-sol-20261004/math/lead/Disagreement.lean

````
import Std

/- A finite weighted coupling. No game semantics are assumed implicitly:
   callers must provide the same scenarios, weights, and payoff-coupling law.
   A true flag `d` permits disagreement; false requires equal payoffs. -/
namespace WaltResearch

def mass {S : Type} (w : S → Nat) (p : S → Bool) : List S → Nat
  | [] => 0
  | x :: xs => (if p x then w x else 0) + mass w p xs

theorem payoff_le_disagreement {S : Type} (w : S → Nat)
    (f g d : S → Bool) (xs : List S)
    (coupling : ∀ x ∈ xs, d x = false → f x = g x) :
    mass w f xs ≤ mass w g xs + mass w d xs := by
  induction xs with
  | nil => simp [mass]
  | cons x xs ih =>
    have hx := coupling x (by simp)
    have ht : ∀ y ∈ xs, d y = false → f y = g y := by
      intro y hy
      exact coupling y (by simp [hy])
    have hi := ih ht
    simp only [mass]
    cases hf : f x <;> cases hg : g x <;> cases hd : d x <;>
      simp_all <;> omega

theorem payoff_sandwich {S : Type} (w : S → Nat)
    (f g d : S → Bool) (xs : List S)
    (coupling : ∀ x ∈ xs, d x = false → f x = g x) :
    mass w f xs ≤ mass w g xs + mass w d xs ∧
    mass w g xs ≤ mass w f xs + mass w d xs := by
  constructor
  · exact payoff_le_disagreement w f g d xs coupling
  · apply payoff_le_disagreement w g f d xs
    intro x hx hd
    exact (coupling x hx hd).symm

theorem maximum_transport {P : Type} (f g : P → Nat) (vf vg error : Nat)
    (attained : ∃ p, f p = vf)
    (target_upper : ∀ p, g p ≤ vg)
    (uniform_error : ∀ p, f p ≤ g p + error) : vf ≤ vg + error := by
  obtain ⟨p, hp⟩ := attained
  have hf := uniform_error p
  have hg := target_upper p
  omega

/- Counts may be maxima over any SAME family of lawful focal policies.
   To instantiate this theorem for best-response values the two error bounds
   must hold for every policy, not just the current incumbent's replay. -/
theorem strict_action_transport (oldA oldB newA newB errA errB : Nat)
    (lowerA : oldA ≤ newA + errA)
    (upperB : newB ≤ oldB + errB)
    (gap : oldB + errA + errB < oldA) : newB < newA := by
  omega

theorem weak_action_transport (oldA oldB newA newB errA errB : Nat)
    (lowerA : oldA ≤ newA + errA)
    (upperB : newB ≤ oldB + errB)
    (gap : oldB + errA + errB ≤ oldA) : newB ≤ newA := by
  omega

/- Exact interval pruning: a later tile can also be discarded at equality.
   An earlier tile requires strict separation to preserve least-tile ties. -/
theorem canonical_interval_pruning (a b va vb lowerA upperB : Nat)
    (la : lowerA ≤ va) (ub : vb ≤ upperB)
    (separated : upperB < lowerA ∨ (upperB ≤ lowerA ∧ a < b)) :
    vb < va ∨ (vb ≤ va ∧ a < b) := by
  omega

#print axioms payoff_le_disagreement
#print axioms payoff_sandwich
#print axioms maximum_transport
#print axioms strict_action_transport
#print axioms weak_action_transport
#print axioms canonical_interval_pruning
end WaltResearch

````

## File: experiments/astra-sol-20261004/phase2/math/sol/FirstDivergence.lean

````
import Init

/- EXPLORATORY finite-horizon replay theorem. State may include a fixed physical
world and complete public history. Focal decisions only receive Observation.
Certificate expands every legal focal action and only the cheap field's edge.
This file does not derive Texas 42 mechanics or completion from a timed program.
-/
namespace WaltResearch.Phase2.Sol

structure Model where
  State : Type
  Action : Type
  Observation : Type
  settled : State → Bool
  focal : State → Bool
  legal : State → List Action
  next : State → Action → State
  observe : State → Observation
  payoff : State → Bool

def prepend (M : Model) (a : M.Action)
    (outcome : List M.Action × Bool) : List M.Action × Bool :=
  (a :: outcome.1, outcome.2)

def replay (M : Model) (field : M.State → M.Action)
    (policy : M.Observation → M.Action) : Nat → M.State → List M.Action × Bool
  | 0, s => ([], M.payoff s)
  | n + 1, s =>
    if M.settled s then ([], M.payoff s)
    else
      let a := if M.focal s then policy (M.observe s) else field s
      prepend M a (replay M field policy n (M.next s a))

def Certificate (M : Model) (cheap target : M.State → M.Action) :
    Nat → M.State → Prop
  | 0, _ => True
  | n + 1, s =>
    if M.settled s then True
    else if M.focal s then
      ∀ a ∈ M.legal s, Certificate M cheap target n (M.next s a)
    else
      cheap s = target s ∧ Certificate M cheap target n (M.next s (cheap s))

def Lawful (M : Model) (policy : M.Observation → M.Action) : Prop :=
  ∀ s, M.settled s = false → M.focal s = true →
    policy (M.observe s) ∈ M.legal s

theorem certified_complete_replay_equal (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (s : M.State)
    (certified : Certificate M cheap target n s) :
    replay M cheap policy n s = replay M target policy n s := by
  induction n generalizing s with
  | zero => rfl
  | succ n ih =>
    cases hs : M.settled s with
    | true => simp [replay, hs]
    | false =>
      cases hf : M.focal s with
      | true =>
        have hc : ∀ a ∈ M.legal s, Certificate M cheap target n (M.next s a) := by
          simpa [Certificate, hs, hf] using certified
        have ha := lawful s hs hf
        have heq := ih (M.next s (policy (M.observe s))) (hc _ ha)
        simpa [replay, hs, hf] using congrArg (prepend M (policy (M.observe s))) heq
      | false =>
        have hc : cheap s = target s ∧
            Certificate M cheap target n (M.next s (cheap s)) := by
          simpa [Certificate, hs, hf] using certified
        have heq := ih (M.next s (cheap s)) hc.2
        simpa [replay, hs, hf, ← hc.1] using congrArg (prepend M (cheap s)) heq

theorem certified_payoff_equal (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (s : M.State)
    (certified : Certificate M cheap target n s) :
    (replay M cheap policy n s).2 = (replay M target policy n s).2 := by
  exact congrArg Prod.snd
    (certified_complete_replay_equal M cheap target policy lawful n s certified)

theorem unequal_payoff_requires_uncertified (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (s : M.State)
    (different : (replay M cheap policy n s).2 ≠ (replay M target policy n s).2) :
    ¬ Certificate M cheap target n s := by
  intro certified
  exact different (certified_payoff_equal M cheap target policy lawful n s certified)

-- Operational callers can pessimistically flag refused/partial worlds.
-- All unflagged worlds must have completed the full certificate obligation.
theorem unflagged_world_coupling (M : Model)
    (cheap target : M.State → M.Action) (policy : M.Observation → M.Action)
    (lawful : Lawful M policy) (n : Nat) (roots : List M.State)
    (bad : M.State → Bool)
    (completed : ∀ s ∈ roots, bad s = false → Certificate M cheap target n s) :
    ∀ s ∈ roots, bad s = false →
      (replay M cheap policy n s).2 = (replay M target policy n s).2 := by
  intro s hs hb
  exact certified_payoff_equal M cheap target policy lawful n s (completed s hs hb)

#print axioms certified_complete_replay_equal
#print axioms certified_payoff_equal
#print axioms unequal_payoff_requires_uncertified
#print axioms unflagged_world_coupling

end WaltResearch.Phase2.Sol

````

## File: experiments/astra-sol-20261004/phase2/TeacherFusion.lean

````
import Std

/- A student receives one constant observation. Safe pays 3 in each hidden
   world. Risk requires a later guess with the SAME observation and pays 4 iff
   correct. Values below are sums over the two equally weighted hidden worlds.
   They are twice expectations, so no floating point/rational axioms are used.
   This concerns clairvoyant continuation labels, NOT fixed lawful rollouts. -/
namespace WaltResearch.Phase2.Lead

def riskPayoff (world guess : Bool) : Nat := if world == guess then 4 else 0
def lawfulRisk (policy : Unit → Bool) : Nat :=
  riskPayoff false (policy ()) + riskPayoff true (policy ())
def safeValue : Nat := 3 + 3
def oracleRisk : Nat := riskPayoff false false + riskPayoff true true

theorem lawful_risk_exact (policy : Unit → Bool) : lawfulRisk policy = 4 := by
  unfold lawfulRisk
  cases policy () <;> simp [riskPayoff]

theorem oracle_label_ranks_risk : safeValue < oracleRisk := by decide

theorem every_lawful_policy_prefers_safe (policy : Unit → Bool) :
    lawfulRisk policy < safeValue := by
  rw [lawful_risk_exact]
  decide

-- Exact constant-input regression to these two oracle labels ranks Risk over
-- Safe, despite every information-measurable continuation doing worse there.
theorem exact_oracle_student_is_wrong (policy : Unit → Bool)
    (predSafe predRisk : Nat) (hs : predSafe = safeValue) (hr : predRisk = oracleRisk) :
    predSafe < predRisk ∧ lawfulRisk policy < safeValue := by
  constructor
  · simpa [hs, hr] using oracle_label_ranks_risk
  · exact every_lawful_policy_prefers_safe policy

-- Legal one-step guesses; training world weights (3,1) and deployment weights
-- (1,3) are both supported on the same two worlds. Lawful inputs alone do not
-- correct that posterior shift. The teacher here is not clairvoyant.
def weightedGuess (wFalse wTrue : Nat) (guess : Bool) : Nat :=
  wFalse * (if guess == false then 1 else 0) +
  wTrue * (if guess == true then 1 else 0)

theorem sampling_distribution_can_reverse_ranking :
    weightedGuess 3 1 true < weightedGuess 3 1 false ∧
    weightedGuess 1 3 false < weightedGuess 1 3 true := by decide

-- Absolute errors for the selected and optimal action imply a 2-epsilon
-- regret bound. This requires action-value accuracy; outcome classification
-- accuracy by itself supplies none of these hypotheses.
theorem chosen_action_regret_bound (qBest qChosen pBest pChosen error : Nat)
    (bestError : qBest ≤ pBest + error)
    (chosenError : pChosen ≤ qChosen + error)
    (greedy : pBest ≤ pChosen) : qBest ≤ qChosen + 2 * error := by omega

#print axioms lawful_risk_exact
#print axioms oracle_label_ranks_risk
#print axioms every_lawful_policy_prefers_safe
#print axioms exact_oracle_student_is_wrong
#print axioms sampling_distribution_can_reverse_ranking
#print axioms chosen_action_regret_bound

end WaltResearch.Phase2.Lead

````

## File: experiments/astra-sol-20261004/tools/run_capped.py

````
#!/usr/bin/env python3
"""Run one foreground POSIX experiment with a short, enforced wall allowance.

Python 3.9+ standard library; macOS/Linux. Children must stay in the process
group (no daemonization, new sessions, or external job submission). A watchdog
cannot enforce a real-time OS guarantee or cancel detached/external GPU jobs.
295 seconds leaves cleanup room below the task's 300-second absolute ceiling.
"""

import argparse
import datetime
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import sys
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seconds", type=float, default=295.0)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if os.name != "posix":
        parser.error("This runner requires macOS/Linux POSIX process groups.")
    if not math.isfinite(args.seconds) or not 0 < args.seconds <= 295:
        parser.error("--seconds must be greater than 0 and at most 295.")
    command = args.command
    if command[:1] == ["--"]:
        command = command[1:]
    if not command:
        parser.error("Supply an executable and its arguments after --.")
    output = args.output_dir.resolve()
    try:
        output.mkdir(parents=True, exist_ok=False)
    except OSError as error:
        parser.error("Use a new writable output directory: " + str(error))

    proc = None
    interrupted = None
    cleanup_errors = []

    def kill_group():
        if proc is not None:
            try:
                os.killpg(proc.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            except OSError as error:
                cleanup_errors.append(str(error))

    def on_signal(signum, _frame):
        nonlocal interrupted
        interrupted = signum
        kill_group()

    original_handlers = {}
    for signum in (signal.SIGINT, signal.SIGTERM, signal.SIGHUP):
        original_handlers[signum] = signal.signal(signum, on_signal)

    started_utc = datetime.datetime.now(datetime.timezone.utc).isoformat()
    started = time.monotonic()
    deadline = started + args.seconds
    status = "spawn_error"
    error_text = None
    child_returncode = None
    exit_code = 125
    try:
        with (output / "stdout.log").open("wb") as stdout, \
                (output / "stderr.log").open("wb") as stderr:
            proc = subprocess.Popen(
                command,
                stdin=subprocess.DEVNULL,
                stdout=stdout,
                stderr=stderr,
                start_new_session=True,
            )
            if interrupted is not None:
                kill_group()
            try:
                child_returncode = proc.wait(
                    timeout=max(0.0, deadline - time.monotonic())
                )
                if interrupted is not None:
                    status = "interrupted"
                    exit_code = 128 + interrupted
                else:
                    status = "completed" if child_returncode == 0 else "failed"
                    exit_code = (child_returncode if child_returncode >= 0
                                 else 128 - child_returncode)
            except subprocess.TimeoutExpired:
                status = "timed_out"
                exit_code = 124
                # No grace period for the workload: terminate the group now.
                kill_group()
            finally:
                # A parent can exit while its children still run. Clean up the
                # group even for normal exit, so background work cannot persist.
                kill_group()
                if proc.poll() is None:
                    try:
                        proc.wait(timeout=3.0)
                    except subprocess.TimeoutExpired:
                        cleanup_errors.append("Process did not reap after SIGKILL.")
                child_returncode = proc.returncode
    except OSError as error:
        error_text = str(error)
        kill_group()
    finally:
        for signum, handler in original_handlers.items():
            signal.signal(signum, handler)

    if cleanup_errors:
        exit_code = 125
    elapsed = time.monotonic() - started
    receipt = {
        "schema": "texas42-partnership-run-v1",
        "started_utc": started_utc,
        "cwd": os.getcwd(),
        "command": command,
        "allowance_seconds": args.seconds,
        "experiment_ceiling_seconds": 300,
        "elapsed_seconds": round(elapsed, 6),
        "status": status,
        "child_pid": None if proc is None else proc.pid,
        "child_returncode": child_returncode,
        "runner_returncode": exit_code,
        "interruption_signal": interrupted,
        "error": error_text,
        "cleanup_errors": cleanup_errors,
        "stdout": "stdout.log",
        "stderr": "stderr.log",
    }
    (output / "run.json").write_text(
        json.dumps(receipt, indent=2) + "\n", encoding="utf-8"
    )
    print("{} in {:.3f}s; record: {}".format(status, elapsed, output / "run.json"))
    return exit_code


if __name__ == "__main__":
    sys.exit(main())

````

## File: experiments/astra-sol-20261004/tools/arena.py

````
#!/usr/bin/env python3
"""Pinned phone-WASM play-only arena. Run ONLY through run_capped.py.

Reuses experiments/partnership/rules.py's independent referee. Every invocation
plays one 28-move game; rotate each source deal through all four seats and swap
the candidate partnership. Aggregate only complete eight-game deal blocks.
"""
import argparse
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path
import random
import select
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT.parent / 'partnership'))
from rules import legal_tiles, winner, trick_points, replay_record, information_state

PHONE = ROOT / 'reference/production-phone/walt-player.wasm'
PUBLIC_POLICY_SEED = 7042104  # independent of hidden source-deal seeds

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def make_request(hands, seat, bidder, plays, decl):
    # Strict information boundary. Referee hands never enter this payload.
    return dict(decl=decl, bid=30, bidder=bidder, seat=seat,
                hand=list(hands[seat]), plays=list(plays), seed=PUBLIC_POLICY_SEED)

def profile(req, partner):
    # Plunge a0d9fa80 native.ts livePlayerCall, thinkDeeper=false, straight 42.
    opening = req['seat'] == req['bidder'] and not req['plays']
    return dict(request=req, worlds=160 if opening else 40,
                partner=False if opening else partner,
                budget_ms=20000 if opening else 14000)

class Worker:
    def __init__(self, wasm):
        self.wasm = wasm
        self.proc = None
        self.buffer = b''
    def start(self):
        self.proc = subprocess.Popen(['node', str(ROOT/'tools/phone_worker.mjs'), str(self.wasm)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, bufsize=0)
    def close(self):
        if self.proc:
            if self.proc.poll() is None: self.proc.kill()
            self.proc.wait(timeout=2)
            for stream in (self.proc.stdin, self.proc.stdout, self.proc.stderr): stream.close()
            self.proc = None
            self.buffer = b''
    def call(self, call, game_deadline):
        started = time.monotonic()
        if self.proc is None: self.start()
        deadline = min(game_deadline, started + call['budget_ms']/1000 + 4)
        self.proc.stdin.write((json.dumps(call)+'\n').encode())
        self.proc.stdin.flush()
        saved, checkpoints = None, 0
        def interrupted(reason):
            self.close()
            if saved is None: raise RuntimeError(reason + '; no retained checkpoint')
            return saved, dict(host_ms=(time.monotonic()-started)*1000,
                checkpoints=checkpoints, interrupted=True, interruption_reason=reason,
                initialization_ms=None)
        while True:
            left = deadline-time.monotonic()
            if left <= 0 or not select.select([self.proc.stdout], [], [], left)[0]:
                return interrupted('Host deadline; last completed decision retained')
            chunk = os.read(self.proc.stdout.fileno(), 65536)
            if not chunk: return interrupted('Phone worker exited')
            self.buffer += chunk
            if len(self.buffer) > 2_000_000: raise RuntimeError('Oversized response')
            while b'\n' in self.buffer:
                line, self.buffer = self.buffer.split(b'\n', 1)
                msg = json.loads(line)
                if 'error' in msg: return interrupted(msg['error'])
                if 'checkpoint' in msg:
                    saved = msg['checkpoint']; checkpoints += 1
                if 'result' in msg:
                    if 'error' in msg['result']: raise RuntimeError(msg['result']['error'])
                    return msg['result'], dict(host_ms=(time.monotonic()-started)*1000,
                        checkpoints=checkpoints, interrupted=False,
                        initialization_ms=msg.get('initialization_ms'))

def run(args):
    output = args.output.resolve()
    if output.exists(): raise ValueError('Refusing to overwrite an existing game')
    output.parent.mkdir(parents=True, exist_ok=True)
    manifest=json.loads((ROOT/'reference/production-phone/manifest.json').read_text())
    assert sha(PHONE)==manifest['wasm_sha256'], 'Pinned phone artifact hash mismatch'
    tiles = list(range(28)); random.Random(args.seed).shuffle(tiles)
    base = [sorted(tiles[7*s:7*s+7]) for s in range(4)]
    hands = [base[(s-args.rotation)%4] for s in range(4)]
    bidder = args.rotation
    candidate_team = (bidder + (args.role == 'defending')) % 2
    baseline, candidate = Worker(PHONE), Worker(args.candidate_wasm)
    workers = [baseline, candidate]
    remaining = [set(h) for h in hands]
    plays, moves, points = [], [], [0, 0]
    leader = bidder
    started = time.monotonic(); deadline = started+args.seconds
    try:
        for _ in range(7):
            trick = []
            for _ in range(4):
                if time.monotonic() >= deadline: raise TimeoutError('Game deadline; incomplete game excluded')
                seat = (leader+len(trick))%4
                is_candidate = seat%2 == candidate_team
                req = make_request(hands, seat, bidder, plays, args.decl)
                call = profile(req, args.candidate_partner if is_candidate else True)
                response, timing = workers[int(is_candidate)].call(call, deadline)
                state = information_state(req)
                legal = legal_tiles(remaining[seat], trick, args.decl)
                if response['choice'] not in legal or sorted(response['legal']) != legal:
                    raise AssertionError('Illegal move or legal-set disagreement')
                if response['points'] != points or response['leader'] != leader:
                    raise AssertionError('Score or leader disagreement')
                if sorted(state['legal']) != legal: raise AssertionError('Information-state legality mismatch')
                tile = response['choice']
                moves.append(dict(call=call, response=response, timing=timing, candidate=is_candidate))
                remaining[seat].remove(tile); plays.extend([seat,tile]); trick.append((seat,tile))
            leader = winner(trick, args.decl); points[leader%2] += trick_points(trick)
    finally:
        for worker in workers: worker.close()
    audited, _, remain, tail = replay_record(hands, plays, args.decl, bidder)
    assert audited == points and sum(points)==42 and not tail and all(not h for h in remain)
    report = dict(schema='walt-astra-sol-game-v1', seed=args.seed, rotation=args.rotation,
        role=args.role, decl=args.decl, bid=30, bidder=bidder, hands=hands,
        policy_seed=PUBLIC_POLICY_SEED, candidate_label=args.candidate_label,
        candidate_partner=args.candidate_partner, candidate_wasm_sha256=sha(args.candidate_wasm),
        phone_wasm_sha256=sha(PHONE), phone_profile='a0d9fa80 native-partner; thinkDeeper=false; straight-42',
        referee_sha256=sha(ROOT.parent/'partnership/rules.py'),
        arena_sha256=sha(__file__), adapter_sha256=sha(ROOT/'tools/phone_worker.mjs'),
        node_version=subprocess.check_output(['node','--version'],text=True,timeout=3).strip(),
        python_version=sys.version, game_seconds=time.monotonic()-started,
        game_limit_seconds=args.seconds, points=points, made=points[bidder%2]>=30,
        moves=moves, complete=True)
    output.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:report[k] for k in ('seed','rotation','role','points','made','game_seconds')}))

def summarize(args):
    rows=[json.loads(p.read_text()) for p in sorted(args.directory.glob('game-*.json'))]
    if not rows: raise ValueError('No complete game receipts')
    signatures={(r['candidate_label'],r['candidate_wasm_sha256'],r['candidate_partner'],
                 r['phone_wasm_sha256'],r['bid'],r['policy_seed'],r['phone_profile'],
                 r['arena_sha256'],r['adapter_sha256'],r['referee_sha256']) for r in rows}
    assert len(signatures)==1, 'Incompatible campaign identities'
    groups={}
    for r in rows:
        assert r['complete'] and len(r['moves'])==28
        key=(r['seed'],r['rotation'],r['role'])
        assert key not in groups, 'Duplicate game'; groups[key]=r
    plan_path=args.directory/'plan.json'
    plan=json.loads(plan_path.read_text()) if plan_path.exists() else None
    attempts=json.loads((args.directory/'attempts.json').read_text()) if plan else []
    seeds=sorted({g['seed'] for g in plan['games']} if plan else {r['seed'] for r in rows})
    cluster=[]; missing=[]; pairs=[]
    for seed in seeds:
        ds=[]
        for rotation in range(4):
            a=groups.get((seed,rotation,'declaring')); b=groups.get((seed,rotation,'defending'))
            if not a or not b: missing.append([seed,rotation]); continue
            assert a['decl']==b['decl']
            d=int(a['made'])-int(b['made']); ds.append(d)
        if len(ds)==4:
            cluster.append(sum(ds)/4);pairs.extend(ds)
    result=dict(games=len(rows), complete_source_deals=len(cluster), missing_pairs=missing,
        pair_wins=pairs.count(1), pair_losses=pairs.count(-1), pair_ties=pairs.count(0),
        planned_games=len(plan['games']) if plan else None,
        attempts=attempts, censored=bool(missing),
        strength_claim=False, uncertainty_unit='whole source deal, averaging four rotations')
    if cluster and not missing:
        mean=sum(cluster)/len(cluster)
        radius=math.sqrt(2*math.log(40)/len(cluster))
        result.update(mean_paired_make_advantage=mean,
            hoeffding_95=[max(-1,mean-radius),min(1,mean+radius)],
            uncertainty_assumption='independent source deals; fixed profiles, bounded differences [-1,1]; conservative smoke interval')
    elif missing:
        result['uncertainty_refusal']='Incomplete/censored panel; no population interval reported'
    for is_candidate,label in ((False,'phone'),(True,'candidate')):
        moves=[m for r in rows for m in r['moves'] if m['candidate']==is_candidate]
        times=sorted(m['timing']['host_ms'] for m in moves)
        result[label]=dict(moves=len(moves), mean_host_ms=sum(times)/len(times),
            median_host_ms=times[len(times)//2], p95_host_ms=times[math.ceil(.95*len(times))-1],
            routes=dict(Counter(m['response']['route'] for m in moves)),
            interruptions=sum(m['timing']['interrupted'] for m in moves),
            budget_ms=sum(m['call']['budget_ms'] for m in moves))
    print(json.dumps(result,indent=2))

def main():
    p=argparse.ArgumentParser(description=__doc__); sp=p.add_subparsers(dest='cmd',required=True)
    r=sp.add_parser('run')
    r.add_argument('--seed',type=int,required=True); r.add_argument('--rotation',type=int,choices=range(4),required=True)
    r.add_argument('--role',choices=['declaring','defending'],required=True)
    r.add_argument('--decl',type=int,choices=list(range(8))+[9],default=6)
    r.add_argument('--candidate-wasm',type=Path,default=PHONE)
    r.add_argument('--candidate-partner',action='store_true')
    r.add_argument('--candidate-label',default='phone-l1-ablation')
    r.add_argument('--seconds',type=float,default=240); r.add_argument('--output',type=Path,required=True)
    s=sp.add_parser('summarize'); s.add_argument('directory',type=Path)
    args=p.parse_args()
    if args.cmd=='run':
        if not 0<args.seconds<=270:p.error('game seconds must be in (0,270]')
        run(args)
    else:summarize(args)
if __name__=='__main__':main()

````

## File: experiments/astra-sol-20261004/tools/phone_worker.mjs

````
// The phone's exact ABI and host clock, with a fresh instance for each decision.
// Compiled-module startup is reused; Node timings are not phone-device timings.
import { readFile } from 'node:fs/promises';
import { createInterface } from 'node:readline';
import { performance } from 'node:perf_hooks';
const started = performance.now();
const module = await WebAssembly.compile(await readFile(process.argv[2]));
const initializationMs = performance.now() - started;
const encoder = new TextEncoder(), decoder = new TextDecoder();
for await (const line of createInterface({ input: process.stdin, crlfDelay: Infinity })) {
  if (!line.trim()) continue;
  try {
    let exports;
    const instance = await WebAssembly.instantiate(module, { walt_host: {
      now_us: () => BigInt(Math.floor(performance.now() * 1000)),
      checkpoint: (ptr, len) => process.stdout.write(JSON.stringify({checkpoint:
        JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer, ptr, len)))}) + '\n'),
    }});
    exports = instance.exports;
    const input = encoder.encode(line), ptr = exports.walt_in_prepare(input.length);
    new Uint8Array(exports.memory.buffer, ptr, input.length).set(input);
    const len = exports.walt_call();
    const result = JSON.parse(decoder.decode(new Uint8Array(exports.memory.buffer, exports.walt_out_ptr(), len)));
    process.stdout.write(JSON.stringify({result, initialization_ms: initializationMs}) + '\n');
  } catch (error) {
    process.stdout.write(JSON.stringify({error: String(error)}) + '\n');
  }
}

````

## File: experiments/astra-sol-20261004/tools/run_panel.py

````
#!/usr/bin/env python3
"""One bounded sequential pilot. All children inherit the outer watchdog group."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
import time

ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--output',type=Path,required=True)
p.add_argument('--seeds',type=int,nargs='+',default=[104200,104201,104202,104203])
p.add_argument('--candidate-partner',action='store_true')
p.add_argument('--candidate-label',default='phone-l1-ablation')
args=p.parse_args()
args.output.mkdir(parents=True,exist_ok=False)
games=[]
for i,seed in enumerate(args.seeds):
    for rotation in range(4):
        roles=['declaring','defending'] if rotation%2==0 else ['defending','declaring']
        for role in roles:games.append(dict(seed=seed,decl=[6,0,7,9][i%4],rotation=rotation,role=role))
plan=dict(scope='play-only; fixed bid30; four rotations and partnership swap per source deal',
    candidate_label=args.candidate_label,candidate_partner=args.candidate_partner,
    games=games, policy_seed=7042104, panel_limit_seconds=280, game_limit_seconds=240)
(args.output/'plan.json').write_text(json.dumps(plan,indent=2)+'\n')
attempts=[];start=time.monotonic()
for game in games:
    left=280-(time.monotonic()-start)
    if left<1:break
    name=f"game-{game['seed']}-r{game['rotation']}-{game['role']}"
    command=[sys.executable,str(ROOT/'tools/arena.py'),'run',
        '--candidate-label',args.candidate_label,'--output',str(args.output/(name+'.json')),
        '--seconds',str(min(240,left))]
    if args.candidate_partner:command.append('--candidate-partner')
    for k,v in game.items():command.extend(['--'+k,str(v)])
    attempt=dict(**game,status='running',command=command)
    attempts.append(attempt)
    (args.output/'attempts.json').write_text(json.dumps(attempts,indent=2)+'\n')
    tick=time.monotonic()
    with (args.output/(name+'.log')).open('w') as log:
        try:
            result=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,timeout=left)
            attempt.update(status='completed' if result.returncode==0 else 'failed',exit_code=result.returncode)
        except subprocess.TimeoutExpired:
            attempt.update(status='timeout',exit_code=124)
    attempt['elapsed_seconds']=time.monotonic()-tick
    (args.output/'attempts.json').write_text(json.dumps(attempts,indent=2)+'\n')
    print(json.dumps({k:v for k,v in attempt.items() if k!='command'}),flush=True)
    if attempt['status']=='timeout':break
if len(attempts)!=len(games) or any(a['status']!='completed' for a in attempts):raise SystemExit(1)

````

## File: experiments/astra-sol-20261004/phase3/results/panel-coverage/summary.json

````
{
  "planned": 15,
  "completed": 15,
  "statuses": {
    "completed": 15
  },
  "source_cases": [
    100,
    101,
    104,
    105,
    107,
    126,
    127,
    128,
    132,
    133,
    165,
    166,
    167,
    206,
    211
  ],
  "tapes": 60,
  "root_action_vectors_compared": 780,
  "all_complete_vectors_equal": true,
  "elapsed_seconds": 7.350645,
  "refused_attempt_seconds": 0,
  "universal_compile_us": 2393919.6240000003,
  "universal_route_us": 52486.78699999999,
  "universal_reduce_13_bids_us": 282707.24899999995,
  "reference_13_bids_us": 906986.2509999998,
  "adaptive_compile_us": 338313.6670000001,
  "adaptive_route_us": 2587.7940000000003,
  "adaptive_reduce_13_bids_us": 231952.792,
  "reference_bid30_us": 120289.79500000003,
  "universal_reduce_bid30_us": 25738.251,
  "adaptive_reduce_bid30_us": 19055.373,
  "universal_nodes": 592717,
  "union_used_coordinates": 58835,
  "unused_coordinates": 533882,
  "union_used_support_incidences": 71816,
  "tape_coordinate_visits": 73104,
  "universal_max_python_object_bytes": 64894815,
  "note": "Python research timing, each vector uses same frozen worlds and tape; no phone strength or speed claim. Includes compile costs explicitly; universal graph may hit node/time cap."
}

````

## File: experiments/partnership/rules.py

````python
"""Independent Straight 42 and own-suit Nel-O rules and information states."""

TILES = [(hi, lo) for hi in range(7) for lo in range(hi + 1)]


def called(tile, decl):
    hi, lo = TILES[tile]
    return decl in (hi, lo) if decl < 7 else decl in (7, 8) and hi == lo


def context(tile, decl):
    return "trump" if called(tile, decl) else TILES[tile][0]


def follows(tile, led, decl):
    return called(tile, decl) if led == "trump" else not called(tile, decl) and led in TILES[tile]


def legal_tiles(hand, trick, decl):
    if not trick:
        return sorted(hand)
    following = [t for t in hand if follows(t, context(trick[0][1], decl), decl)]
    return sorted(following or hand)


def winner(trick, decl):
    led = context(trick[0][1], decl)

    def strength(play):
        t = play[1]
        hi, lo = TILES[t]
        tier = 2 if decl != 8 and called(t, decl) else 1 if follows(t, led, decl) else 0
        rank = hi if hi == lo and decl in (7, 8) else 12 if hi == lo else hi + lo
        return tier, rank

    return max(trick, key=strength)[0]


def trick_points(trick):
    return 1 + sum(sum(TILES[t]) if sum(TILES[t]) in (5, 10) else 0 for _, t in trick)


def active_actor(leader, offset, bidder, contract=None):
    actor = leader
    for _ in range(offset):
        actor = (actor + 1) % 4
        if contract == 'nello' and actor == (bidder + 2) % 4:
            actor = (actor + 1) % 4
    return actor


def replay_record(hands, record, decl, bidder, contract=None):
    remaining = [set(h) for h in hands]
    assert sorted(t for hand in hands for t in hand) == list(range(28))
    points = [0, 0]
    lead, trick, completed = bidder, [], 0
    for actor, tile in zip(record[::2], record[1::2]):
        assert not (contract == 'nello' and completed and lead == bidder)
        assert actor == active_actor(lead, len(trick), bidder, contract)
        assert tile in legal_tiles(remaining[actor], trick, decl)
        remaining[actor].remove(tile)
        trick.append((actor, tile))
        if len(trick) == (3 if contract == 'nello' else 4):
            lead = winner(trick, decl)
            points[lead % 2] += trick_points(trick)
            trick = []
            completed += 1
    return points, lead, remaining, trick


def _request_fields(req):
    if not isinstance(req, dict):
        raise ValueError("request must be an object")
    for key in ("decl", "bid", "bidder", "seat"):
        if type(req.get(key)) is not int:
            raise ValueError(key + " must be an integer")
    decl, bid, bidder, viewer = (req[k] for k in ("decl", "bid", "bidder", "seat"))
    contract = req.get('contract')
    if 'contract' in req and contract != 'nello':
        raise ValueError('unsupported contract')
    if ((contract == 'nello' and (decl != 8 or not 1 <= bid <= 9))
            or (contract is None and (decl not in (*range(8), 9) or not 30 <= bid <= 42))):
        raise ValueError("invalid declaration or bid")
    if bidder not in range(4) or viewer not in range(4):
        raise ValueError("invalid bidder or seat")
    if contract == 'nello' and viewer == (bidder + 2) % 4:
        raise ValueError('inactive seat cannot play')

    hand = req.get("hand")
    plays = req.get("plays", [])
    if not isinstance(hand, list) or len(hand) != 7:
        raise ValueError("hand needs seven tile ids")
    if any(type(tile) is not int or tile not in range(28) for tile in hand):
        raise ValueError("hand needs tile ids 0..27")
    if len(set(hand)) != 7:
        raise ValueError("duplicate hand tile")
    if not isinstance(plays, list) or len(plays) % 2 or len(plays) > (42 if contract == 'nello' else 56):
        raise ValueError("invalid record length")
    if any(type(value) is not int or value < 0 for value in plays):
        raise ValueError("record needs unsigned integers")
    return decl, bidder, viewer, hand, plays


def _hidden_completion_exists(viewer, own_remaining, played, sizes, void_tiles):
    """Exact three-hand capacity DP: at most 21 tiles and 8^3 states."""
    others = [seat for seat in range(4) if seat != viewer]
    capacities = tuple(sizes[seat] for seat in others)
    unseen = sorted(set(range(28)) - played - own_remaining)
    if len(unseen) != sum(capacities):
        return False

    states = {(0, 0, 0)}
    for tile in unseen:
        next_states = set()
        for counts in states:
            for slot, seat in enumerate(others):
                if counts[slot] < capacities[slot] and tile not in void_tiles[seat]:
                    updated = list(counts)
                    updated[slot] += 1
                    next_states.add(tuple(updated))
        states = next_states
        if not states:
            return False
    return capacities in states


def information_state(req):
    """Derive a lawful current state from one original hand and public play."""
    decl, bidder, viewer, original_hand, record = _request_fields(req)
    contract = req.get('contract')
    own_remaining = set(original_hand)
    played = set()
    void_tiles = [set() for _ in range(4)]
    play_counts = [0, 0, 0, 0]
    points = [0, 0]
    leader, trick, completed = bidder, [], 0

    for actor, tile in zip(record[::2], record[1::2]):
        if contract == 'nello' and completed and leader == bidder:
            raise ValueError('record continues after Nel-O was set')
        if actor not in range(4) or tile not in range(28):
            raise ValueError("invalid record actor or tile")
        if actor != active_actor(leader, len(trick), bidder, contract):
            raise ValueError("record violates turn order")
        if tile in played:
            raise ValueError("record repeats a tile")
        if play_counts[actor] >= 7:
            raise ValueError("a seat plays more than seven tiles")
        if tile in void_tiles[actor]:
            raise ValueError("record contradicts a revealed void")

        if actor == viewer:
            if tile not in own_remaining:
                raise ValueError("own historical tile is not in the original hand")
            if tile not in legal_tiles(own_remaining, trick, decl):
                raise ValueError("own historical play is illegal")
            own_remaining.remove(tile)
        elif tile in original_hand:
            raise ValueError("another seat played own tile")

        if trick:
            led = context(trick[0][1], decl)
            if not follows(tile, led, decl):
                void_tiles[actor].update(t for t in range(28) if follows(t, led, decl))

        played.add(tile)
        play_counts[actor] += 1
        trick.append((actor, tile))
        if len(trick) == (3 if contract == 'nello' else 4):
            leader = winner(trick, decl)
            points[leader % 2] += trick_points(trick)
            trick = []
            completed += 1

    if completed == 7 or (contract == 'nello' and completed and leader == bidder):
        raise ValueError("hand is complete")
    if viewer != active_actor(leader, len(trick), bidder, contract):
        raise ValueError("not this seat's turn")

    sizes = [7 - count for count in play_counts]
    if len(own_remaining) != sizes[viewer]:
        raise ValueError("own hand and public record disagree")
    if not _hidden_completion_exists(viewer, own_remaining, played, sizes, void_tiles):
        raise ValueError("public record has no hidden-hand completion")

    legal = legal_tiles(own_remaining, trick, decl)
    if not legal:
        raise ValueError("position has no legal move")
    return {"legal": legal, "leader": leader, "points": points, "trick": completed + 1,
            **({'contract': 'nello', 'inactive': (bidder + 2) % 4} if contract else {})}

````

## Standalone fixture: plan.json

````json
{
  "schema": "walt-lawful-tape-coordinate-plan-v1",
  "fixtures": [
    {
      "id": 126,
      "seed": 680126,
      "ply": 16,
      "hands": [
        [
          2,
          4,
          6,
          8,
          11,
          13,
          16
        ],
        [
          5,
          12,
          17,
          19,
          21,
          24,
          26
        ],
        [
          0,
          7,
          9,
          14,
          15,
          20,
          25
        ],
        [
          1,
          3,
          10,
          18,
          22,
          23,
          27
        ]
      ],
      "points": [
        18,
        1
      ],
      "eligible": true,
      "request": {
        "decl": 0,
        "bid": 30,
        "bidder": 2,
        "seat": 1,
        "hand": [
          5,
          12,
          17,
          19,
          21,
          24,
          26
        ],
        "plays": [
          2,
          20,
          3,
          18,
          0,
          16,
          1,
          26,
          2,
          0,
          3,
          1,
          0,
          6,
          1,
          21,
          2,
          15,
          3,
          10,
          0,
          2,
          1,
          19,
          2,
          7,
          3,
          23,
          0,
          13,
          1,
          24
        ],
        "seed": 7042104
      }
    }
  ],
  "world_count": 40,
  "tape_seeds": [
    790001,
    790002,
    790003,
    790004
  ],
  "bids": [
    30,
    31,
    32,
    33,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41,
    42
  ],
  "compilation_node_cap": 100000,
  "compilation_seconds": 20,
  "panel_seconds": 270,
  "source_panel_sha256": "b5184a63b33ed219076c2642e38d61795af3feac91023f4b35d68e8e047d1836",
  "python": "3.9.6 (default, May 22 2026, 11:13:45) \n[Clang 21.0.0 (clang-2100.1.1.101)]",
  "source_hashes": {
    "run_panel.py": "21b69fe2e7ffea26ba9c959518bd12f891eef369c2872002c2bbad1ad2646359",
    "compiled_tape.py": "77e30a0df02852afd4730001e3996b3d68834966f73bbf109de373c4416e2fbd"
  },
  "semantics": "Fixed 64-bit nature tapes and same sampled hidden worlds; lawful shared focal decisions. Not exact integration over hidden-world posterior or nature, and not production Walt field."
}
````

## Standalone fixture: case-0126.json

````json
{
  "id": 126,
  "ply": 16,
  "decl": 0,
  "worlds": [
    [
      2576,
      135200,
      33571072,
      138412040
    ],
    [
      33556992,
      135200,
      16648,
      138412048
    ],
    [
      134219792,
      135200,
      776,
      37765120
    ],
    [
      134218256,
      135200,
      4194568,
      33572864
    ],
    [
      37748992,
      135200,
      16912,
      134219784
    ],
    [
      138412544,
      135200,
      18688,
      33554456
    ],
    [
      134234128,
      135200,
      2816,
      37748744
    ],
    [
      33571072,
      135200,
      134220288,
      4194328
    ],
    [
      16656,
      135200,
      33556992,
      138412040
    ],
    [
      134220288,
      135200,
      280,
      37765120
    ],
    [
      134234128,
      135200,
      33555200,
      4196360
    ],
    [
      138428416,
      135200,
      784,
      33556488
    ],
    [
      16912,
      135200,
      4196608,
      167772168
    ],
    [
      4194576,
      135200,
      167772672,
      18440
    ],
    [
      16912,
      135200,
      134220032,
      37748744
    ],
    [
      17152,
      135200,
      167772168,
      4196368
    ],
    [
      2320,
      135200,
      138412544,
      33570824
    ],
    [
      4196864,
      135200,
      167772416,
      16408
    ],
    [
      16912,
      135200,
      4194568,
      167774208
    ],
    [
      134218496,
      135200,
      2072,
      37765120
    ],
    [
      33571072,
      135200,
      134218256,
      4196360
    ],
    [
      134234624,
      135200,
      2320,
      37748744
    ],
    [
      134218496,
      135200,
      33572864,
      4194328
    ],
    [
      33572864,
      135200,
      784,
      138412040
    ],
    [
      167772176,
      135200,
      776,
      4212736
    ],
    [
      33556992,
      135200,
      4194568,
      134234128
    ],
    [
      134234368,
      135200,
      2568,
      37748752
    ],
    [
      134234128,
      135200,
      2816,
      37748744
    ],
    [
      4196864,
      135200,
      280,
      167788544
    ],
    [
      4196864,
      135200,
      16656,
      167772168
    ],
    [
      4195072,
      135200,
      167774208,
      16408
    ],
    [
      134220032,
      135200,
      536,
      37765120
    ],
    [
      33556992,
      135200,
      16656,
      138412040
    ],
    [
      4195072,
      135200,
      134217752,
      33572864
    ],
    [
      784,
      135200,
      167774208,
      4210696
    ],
    [
      4210944,
      135200,
      33554952,
      134219792
    ],
    [
      33571328,
      135200,
      134217992,
      4196368
    ],
    [
      33571072,
      135200,
      4194832,
      134219784
    ],
    [
      33556736,
      135200,
      134218256,
      4210696
    ],
    [
      134234368,
      135200,
      33554960,
      4196360
    ]
  ],
  "request": {
    "decl": 0,
    "bid": 30,
    "bidder": 2,
    "seat": 1,
    "hand": [
      5,
      12,
      17,
      19,
      21,
      24,
      26
    ],
    "plays": [
      2,
      20,
      3,
      18,
      0,
      16,
      1,
      26,
      2,
      0,
      3,
      1,
      0,
      6,
      1,
      21,
      2,
      15,
      3,
      10,
      0,
      2,
      1,
      19,
      2,
      7,
      3,
      23,
      0,
      13,
      1,
      24
    ],
    "seed": 7042104
  },
  "tapes": [
    {
      "seed": 790001,
      "tape": [
        [
          2976701151674331434,
          8925282868940396140,
          12904000043716393900,
          14290399319215578336,
          13216346440311087445,
          7424517113220375608,
          8472658571139651957,
          8612990748893398642,
          17968689430825234408,
          3130089805365681938,
          8631076972017576571,
          15482462209821947267
        ],
        [
          18000582124925108359,
          6709760388236845606,
          15639640582202119110,
          7494883521575865544,
          6894300169660370338,
          1900852943720988689,
          8858005299652480777,
          5486708327629664816,
          8289779731832982864,
          916773974124580956,
          12989534122370554900,
          18036064339461008471
        ],
        [
          13996806781889801539,
          17136633490455035282,
          9881897933587793412,
          4981704159783306541,
          14071700347060382200,
          12516293547349230378,
          15893669944598079635,
          12271541627174771146,
          2325125626834361454,
          13209281184765257812,
          590365713661099932,
          9265663613529698536
        ],
        [
          10332678980247955450,
          1429741591970927387,
          2774866912029155928,
          11623932870087680114,
          13647994477961220566,
          262792349938041980,
          16042123491589326304,
          17266439710281208250,
          2591356576302157564,
          10496997224489787128,
          3881685424844855092,
          6536066722956657302
        ],
        [
          13902797602725840362,
          15069183270430432880,
          14037964945931244359,
          9204276806226113371,
          4191263633128741188,
          16504321778930701417,
          9361738414962045092,
          2983009160488077331,
          694162081972858011,
          9272124860737306730,
          17480944253546589932,
          14531812081384617284
        ],
        [
          10299524609725976811,
          8802072736262343981,
          17423328914772979835,
          16867803402625619284,
          2406769827193406729,
          6778299177382855512,
          9774404720958088513,
          16023300354568245800,
          15785442903752832461,
          10788454094794327906,
          6525537548064731637,
          13274774632765649423
        ],
        [
          1559045919943473616,
          5548723568495160785,
          11170414582747202195,
          9501564386883463217,
          9481350813940893912,
          5411933813694992130,
          17748338391647280177,
          7858926483529391890,
          10660693561660535614,
          1658346828646500596,
          9690129317837824083,
          551601597459904605
        ],
        [
          16540358873575730022,
          890805972422348461,
          9642880626632763604,
          13029062429000287932,
          17519922741738498995,
          15511785231356790035,
          11065869402096957271,
          11369001385621542954,
          14583354395969451733,
          7312839113574008261,
          14069159988898939787,
          15198637792837207099
        ],
        [
          13251722837060691570,
          171875814966492293,
          8050383639563218410,
          13880466609886985459,
          17382122053525756907,
          3979357926385706307,
          6105398901151291049,
          3033185791177293945,
          16828158388240569545,
          18444715422264660348,
          16382025445196203569,
          2704071086126611927
        ],
        [
          3610086588082273796,
          242280312095333881,
          2066909179613613591,
          16271170875514866060,
          2957633431507910329,
          11524389786566877458,
          9271189703616688969,
          7541255437340947042,
          11079641689207185067,
          16423349719475869593,
          11554984384795581071,
          17929500655974885709
        ],
        [
          17735125464220874173,
          13017177679462352011,
          16840878907221295890,
          4787450797121606517,
          3174060488598353370,
          10524271918713905905,
          7604063830194310950,
          18384591686779705211,
          12231725732045553462,
          5227646429573220743,
          8661715043066637527,
          3163231228437559865
        ],
        [
          15358619366116457242,
          2484504475993119842,
          17292284849929597559,
          8096584813030965404,
          12840364882551162599,
          3142305895862572223,
          16408018399559378120,
          5924681665323343873,
          1720654263788731638,
          16708271030388851338,
          10724267552609019932,
          1794084462926298636
        ],
        [
          716073685336567606,
          7388982741020449553,
          5521915492647149135,
          162052029137375591,
          5412147125880055712,
          18025093651408190268,
          10067113300151805225,
          15028874545863611488,
          4606775301079832606,
          9407594332420858823,
          8963559643278550470,
          451389219274143615
        ],
        [
          3296710325051212342,
          6648845964563466414,
          8276957908940125786,
          6636292321003409811,
          15840837506492113539,
          4490177438502794425,
          17647165921107702018,
          13638442898016381817,
          15028886378979215818,
          2000533355492152219,
          15512786163818139003,
          1262943430555627965
        ],
        [
          13454712383407334782,
          12229018258827539114,
          5186354176139558148,
          5196595384823091398,
          5915370920495821506,
          625632521915212571,
          9275214540055880243,
          13260406053337778887,
          8315026633175968851,
          5831757009620807652,
          4292872944422269831,
          9742166033979054545
        ],
        [
          4113532252821011175,
          17048736142180995483,
          16974781928829988205,
          9846363780741764769,
          11449941566424944112,
          16702965747676531223,
          15992214850390833903,
          6986686012788901162,
          17998368411692819352,
          5802738590788034650,
          13457053654523946025,
          18413893052976450100
        ],
        [
          18070340690109677728,
          8074580357835189898,
          1182044692035332556,
          10105055508758995716,
          4627509891658496772,
          7567286847453025559,
          6181529722562481538,
          13132687106145566157,
          8346168836513143988,
          3800388454438702366,
          16790980538135201276,
          4139824055714034263
        ],
        [
          5276353881114280537,
          820995378127255420,
          3652593261442533791,
          14125850614819574838,
          15790773198703016328,
          11536307394470854403,
          13000271166776151100,
          12615808476434172946,
          9277266905152642461,
          12118163275990797674,
          191867898493266369,
          6633458655654575008
        ],
        [
          9293702864496099615,
          14630090373113670925,
          764821920419650396,
          8630654106292913314,
          10119156260782181854,
          7906535104568345798,
          5941796167691352986,
          9350825301451946355,
          6744748910792735167,
          2153130767529845007,
          18041935962172527759,
          18372801614818896083
        ],
        [
          17544841037425087435,
          6182852765625123941,
          1435903500010456576,
          409012414073814353,
          4375413909415785526,
          14696961655669232741,
          13639566356300281391,
          13719770450203088527,
          14157498351576837879,
          2070691268602908477,
          9793304208723070768,
          8641143505531150531
        ],
        [
          14946649428866043304,
          10843688733206961774,
          9225437409055311389,
          3340249913144345927,
          289931131554927471,
          8259909139827849020,
          17007053591103402048,
          3035962401264533386,
          14701189533830179330,
          8439895531172494682,
          5158942482770766896,
          10802519854829008444
        ],
        [
          4503725250215441887,
          9364036256839154900,
          3834171723004845912,
          13560646878917473956,
          10882973501461156902,
          16012057359382467723,
          11236808901861833361,
          17245178549389025584,
          6426177723228563540,
          2583086518448575038,
          13307544990138779578,
          5974285651323546219
        ],
        [
          7881964799949279600,
          2183669095943233236,
          1182281453493677768,
          14141436239977123812,
          7631210261402379177,
          7575570386621487001,
          1799072359500624540,
          474297773787638633,
          13768720747199401564,
          4825292635785034240,
          10359572602465049395,
          12812454168499178313
        ],
        [
          15001498332991049095,
          87981160623484910,
          9908532616698100892,
          6201441640614972489,
          17486050222722959529,
          1464669759007304455,
          12624519109494588343,
          3347212930887072905,
          16952381447691037917,
          3661672295393865891,
          9402802438404797199,
          8181608211076345496
        ],
        [
          5342854399699182402,
          11382839312115083052,
          6491902034264945972,
          16533052338551494297,
          9375264923611606567,
          14306376415266712586,
          10837231972039663542,
          18174774631793148396,
          17343160806895408218,
          13785997124676155952,
          8014775646399083908,
          1770605896153776645
        ],
        [
          11532022463211854426,
          3680478402166863391,
          18221304259817299673,
          3754689845267832643,
          6771681523446883664,
          15951268954317434722,
          4835485951276007863,
          6934600671119715898,
          6572128952252418929,
          3753057167858024384,
          11285714488078713231,
          4922060505548421508
        ],
        [
          75310851200637639,
          3671245166853726058,
          8454857002915802813,
          4666712190291210412,
          15288561702873629856,
          12277133646573030453,
          10996325993924305545,
          9375759441495106014,
          3329841474959186054,
          9207303035998301792,
          8813159381265450323,
          5279074176231428040
        ],
        [
          385889834354696732,
          15791888230839268645,
          2886231135995421282,
          6975325733456314423,
          17127304538946467528,
          14949499505520738948,
          10375924003427127999,
          11613062641139551397,
          13319875170312158316,
          8097263605206645962,
          12480052261602266798,
          1821901085405154179
        ],
        [
          5396948629527815229,
          4097978858971858865,
          18341793113528702249,
          14716283489762660505,
          16926052661514762991,
          148397759869130928,
          5624288597862757639,
          7253101734696822970,
          9497264655809808184,
          1448200560967712190,
          14250685903071336277,
          13873190633369992580
        ],
        [
          9597664863546881458,
          13227948708116685632,
          16907416644335633107,
          11235586974246532899,
          13971836666562265678,
          8461109833711716760,
          15121363824518506862,
          12638214864106543648,
          9464880510755748928,
          11815601124163322432,
          7827710835563826640,
          10602088217923247370
        ],
        [
          15876951697419243704,
          7124172952013644720,
          16418281576335452677,
          312038737716101995,
          12481732110122550765,
          7158394282531829380,
          8140409738342109098,
          17246580643345324584,
          15933868669149090671,
          10224427228950069514,
          9407599562398083844,
          7127315631868873920
        ],
        [
          16362618445359578803,
          6527169533152483405,
          10865198214242299649,
          9172029945873871134,
          5733701294854092259,
          14994882444158520822,
          14153977534025072160,
          8984624911775005589,
          6279434344490266678,
          16763418902972224311,
          8514211352974300909,
          16647647549471078655
        ],
        [
          13718364211242110383,
          18179487841136725890,
          361161115869844363,
          17709135208301784935,
          6707293594677596236,
          11163082606071234922,
          4124335787907829107,
          13074576708846297124,
          8194687259469479132,
          12826025096043360531,
          11346666192287559360,
          6103712705753781824
        ],
        [
          7972670901278832159,
          1676308810756394795,
          6631249012702032714,
          3260538448147992866,
          14427145472125247491,
          11269305645980262482,
          13066055802898145782,
          14762205545415036605,
          10169266384743800625,
          9962263091167402124,
          201579132043115190,
          12293710737630715343
        ],
        [
          11949822432903518269,
          11288946888421509708,
          2464699227798842603,
          9662858546331778313,
          1872675344632923510,
          11703701341094396696,
          2766925751403704098,
          6163915045722484383,
          5504854712316792227,
          6130503065453095412,
          11289150629824506275,
          3745291124670553677
        ],
        [
          4936308751582104564,
          1903222563894473268,
          8631683943500295270,
          4066352479858229799,
          1393680961150010333,
          873377461115824836,
          4354031195852439370,
          12596614395690457316,
          9159989171497295636,
          14062269556629017379,
          12901220150148561336,
          8394685394032306618
        ],
        [
          15599349526764239160,
          3555078474158189247,
          12062274791752232492,
          4976043157194253086,
          13516895440791338645,
          5063279992465431974,
          8234056856347536689,
          13779974197992734791,
          9604408567211513745,
          6623717052663683806,
          1929956648037825196,
          11879973355404626378
        ],
        [
          13162667007511403093,
          12128044099557537392,
          9418939919460609936,
          13821742955849920951,
          4322169376413489613,
          12143234887741447487,
          3252034536410708741,
          3450499805529863990,
          9504773119836651818,
          2767471877746694754,
          5416820252243968662,
          6007655242626762805
        ],
        [
          11341171336063754868,
          15625709930459043682,
          8599357864262733953,
          12319395509278567626,
          5752222171204820514,
          1662856557639812976,
          10242581531539862436,
          7951237215596458817,
          6240115541137966248,
          11645947281387397601,
          15966423685588437921,
          16058624594962002885
        ],
        [
          1266167155381268151,
          876938375201022346,
          5428428960254178839,
          7939104690749004439,
          8850090640103834151,
          3226230058387477944,
          13143448120406571625,
          4305704807094972537,
          12615203402387879379,
          4223042726194321472,
          710129823381031228,
          15140560826293338686
        ]
      ],
      "reference_13_bids_us": 38994.209,
      "reference_calls": {
        "30": 962,
        "31": 784,
        "32": 784,
        "33": 784,
        "34": 784,
        "35": 940,
        "36": 621,
        "37": 621,
        "38": 621,
        "39": 621,
        "40": 645,
        "41": 581,
        "42": 3
      },
      "universal_route_us": 1410.166,
      "universal_reduce_13_bids_us": 7398.0,
      "adaptive_compile_us": 28132.417,
      "adaptive_route_us": 98.667,
      "adaptive_reduce_13_bids_us": 5869.625,
      "adaptive_statistics": {
        "nodes": 1867,
        "edges": 1866,
        "leaves": 229,
        "support_incidences": 2249,
        "python_object_bytes": 1441047,
        "max_depth": 12
      },
      "reference_bid30_us": 3049.041,
      "universal_reduce_bid30_us": 751.875,
      "adaptive_reduce_bid30_us": 481.791,
      "full_reference_bid30_us": 5440.75,
      "full_reference_bid30_calls": 1866,
      "values": {
        "30": {
          "5": 30,
          "12": 14,
          "17": 30
        },
        "31": {
          "5": 33,
          "12": 15,
          "17": 32
        },
        "32": {
          "5": 33,
          "12": 15,
          "17": 32
        },
        "33": {
          "5": 33,
          "12": 15,
          "17": 32
        },
        "34": {
          "5": 33,
          "12": 15,
          "17": 32
        },
        "35": {
          "5": 34,
          "12": 21,
          "17": 33
        },
        "36": {
          "5": 38,
          "12": 29,
          "17": 36
        },
        "37": {
          "5": 38,
          "12": 29,
          "17": 36
        },
        "38": {
          "5": 38,
          "12": 29,
          "17": 36
        },
        "39": {
          "5": 38,
          "12": 29,
          "17": 36
        },
        "40": {
          "5": 38,
          "12": 30,
          "17": 36
        },
        "41": {
          "5": 39,
          "12": 32,
          "17": 37
        },
        "42": {
          "5": 40,
          "12": 40,
          "17": 40
        }
      },
      "all_values_equal": true
    },
    {
      "seed": 790002,
      "tape": [
        [
          16598829954277595593,
          9328340697610682351,
          13732606910633596437,
          2300666813844194980,
          4200425610791491016,
          5065687394148015593,
          18279588710685272110,
          342634346255301340,
          7326122789534144681,
          7089911557143794878,
          2422951671054871125,
          11836136758789690000
        ],
        [
          13339985261146020020,
          5834885306907026916,
          14602910671271754638,
          4714551204712628726,
          2470643730012944684,
          1458736399217024174,
          16568237830779189133,
          17467019985891360856,
          3524933396677915446,
          1282467800077595583,
          7566599993803240302,
          6634689720004021869
        ],
        [
          12919586397265214864,
          7931059747321316461,
          14382630267452206485,
          8044242174088590310,
          6044796075840259549,
          13855219782731858006,
          12530282855472579792,
          5403790191502730126,
          16882665056065692918,
          13216200379883570554,
          1202334885068201577,
          8064407551845391927
        ],
        [
          995186690268430951,
          13559341096961417560,
          507572612208908694,
          13019935016004414230,
          7577752286699383754,
          1158804450929388306,
          13841554264709209297,
          17920160427633000136,
          16263280868204322855,
          13755949015289853867,
          1703784732596407154,
          5441332036581689794
        ],
        [
          1960290253124130797,
          1230617693008038843,
          2193537268932847702,
          9309080033343463852,
          5505031503670994472,
          13215684489386653151,
          6582287368585227720,
          16326792473573650124,
          3780523100290286431,
          6484586353277199182,
          10520768267469226613,
          5300449284543699350
        ],
        [
          7421757426129084461,
          2878674659595027816,
          7015922039603808800,
          14724851358854950005,
          2090249969114732587,
          10076857325907337364,
          6539298605839949684,
          2089346201120464765,
          6754509342506164429,
          7884678114059699539,
          4051178605364562793,
          12551618270138829876
        ],
        [
          7850473041714492693,
          12612011853517144992,
          16728396298234558773,
          11548929578030053899,
          10065072878178255087,
          17717622354706221877,
          7947911524340773876,
          17674093984839942051,
          2567180490871537767,
          7776492378094019633,
          3976290383547528814,
          15306437379709587357
        ],
        [
          16420287601304061170,
          15521287647811970427,
          14962113450307913159,
          8123154463707973855,
          13852651515191506423,
          1156040852109648685,
          978580509041712121,
          5548124706973194078,
          12667420267646887714,
          1230146656888781845,
          5452183077154017982,
          9021351032050821280
        ],
        [
          10305738900160937292,
          5105945223950376633,
          9588012794602082951,
          6218498220945040631,
          16494764687365846928,
          15134550891499494403,
          18247624383415309540,
          8372433377275874631,
          749311773443160417,
          7200647130126667592,
          11705658323526276871,
          5595283672110209397
        ],
        [
          2844332108618240861,
          6002867694250833043,
          9105254829087052202,
          2025727226938370172,
          5744571378527990512,
          6128987899967320715,
          8772106157159207473,
          13632770613191345487,
          13157695830629280656,
          16349052817500643717,
          14649372027084121517,
          14161706737022524040
        ],
        [
          7359345803010403639,
          16862557199233737143,
          6756033449009098805,
          3470885607772946450,
          17668982334056313529,
          11533942134455453034,
          5522903227889649166,
          11507049675852536939,
          10276545889293277801,
          8075972896447708915,
          13186739010953553699,
          14160766731349759004
        ],
        [
          3996372841733398016,
          13188266400335859562,
          6637979078922639694,
          1970547873910352427,
          15149742102584258978,
          13015555357578165070,
          5101347885799000272,
          12819093300970805512,
          9788396773346119470,
          17434668421594117752,
          13200745310132155169,
          9048617263002062562
        ],
        [
          8296925735588386564,
          6629376195185241339,
          5344860341318284494,
          10511982411366861217,
          1486825191829715793,
          9501396729415946585,
          6916615944076615959,
          11373157873056307616,
          4360706823964937246,
          9322865019925512746,
          5009734096066814041,
          4059733805087074906
        ],
        [
          2287541667185598049,
          1903200201709126956,
          6034169692096310849,
          35459497861006107,
          12506352092058392689,
          5191321856276055887,
          5245221132087956123,
          8665859185873906183,
          2221729025840469835,
          10911989443626030939,
          2074761083189666687,
          1926007798261916769
        ],
        [
          442281794844409500,
          2257683755458285875,
          7373195992768396565,
          4683893613175165365,
          7788632946637752257,
          5376216280111543869,
          17118646506407961249,
          16688985523152225772,
          2887997128208240723,
          526834216117064172,
          3564624001686472075,
          4149242701116646998
        ],
        [
          13487192879851788656,
          2732609636081862194,
          15552134680619783851,
          18428890067672258693,
          14391959287337636940,
          14345536881038335636,
          15896686815402738816,
          12741689702906289494,
          15756008691848080780,
          5567277343332885784,
          8284366624991816235,
          13593680293585823798
        ],
        [
          13820116878451460213,
          13663146098043136649,
          700724714818381210,
          12616915225606961484,
          14528349465542230026,
          2084046363129265654,
          966915252455386050,
          14790237466872669572,
          1332033672363182589,
          3584825695838938825,
          8768415159171933965,
          17754353265605931759
        ],
        [
          14680519941212575322,
          10329478790540204031,
          6115189180066794162,
          10467860353365476245,
          2548183353303442950,
          11832868284195840252,
          5270280184909298412,
          6682110322126180346,
          13169403951108256957,
          16054515801418837717,
          6782531475604743770,
          5019991321732840401
        ],
        [
          13805128845351165009,
          10568709482460351306,
          1632629686159413567,
          2122450722865796693,
          14397930460801124467,
          6638328500366491471,
          17208157827279854620,
          5037626243299435309,
          16119647756516099336,
          11549510920740638678,
          17303634527813833138,
          6484460961274641062
        ],
        [
          1198518762605580123,
          2708354773240680127,
          4851422454963087970,
          11660213709063196228,
          2606241586605607702,
          15417199676932972233,
          17404115386611865752,
          18106403738606585196,
          8127992165481523525,
          8058909210457532418,
          1535360654426931289,
          10824939611462229284
        ],
        [
          6018964398989425633,
          6003702889278058134,
          10511561229100912688,
          17374764485582235263,
          16684449128145799165,
          6983340411234326962,
          9274312169871066916,
          3763853982339473598,
          10120925942762638813,
          17209960024145447281,
          8418680552274124592,
          854806939654020664
        ],
        [
          5115103211288267430,
          17570953671377025207,
          817008065622897447,
          13691676287054583691,
          3851328541365544190,
          1689957290129729940,
          1479695037905818182,
          18263658493268543716,
          4874885530977154803,
          4380025450096820751,
          9564278533917555756,
          2015298872059694208
        ],
        [
          11932488561470002666,
          12198819226954546618,
          3694187162561261724,
          2384381545719189701,
          18425647327834625229,
          1558921298825511429,
          17028424099077443229,
          13017495533376806241,
          8741642276473024436,
          2931866441356078122,
          1443303372571789447,
          11449010872382646445
        ],
        [
          10919501764872409250,
          5276480234947304595,
          5292827631688987246,
          2249387759202445057,
          12033339529015068902,
          14223813155219630976,
          9888799569671346882,
          13282799532166228061,
          9431097650471497412,
          13644837154495127093,
          9752603240896895549,
          10546553419324668942
        ],
        [
          12574777618314842305,
          14099005760634761825,
          165602726558942614,
          13743902370028056689,
          861050084023091373,
          7328534967303983048,
          1164463703043527507,
          13575778102765120227,
          3441222904803731379,
          18311564297041378795,
          2994648049693958352,
          4209056354072906717
        ],
        [
          16227404491731741957,
          2120614146229746834,
          629287637878593953,
          16505204725391485700,
          4481271698472444782,
          16193037004012555072,
          16558935938532692631,
          12023718725114778469,
          1038054101916085358,
          16808790482412689126,
          10677872654805177485,
          4171019681857316715
        ],
        [
          15606479745082479568,
          16970033296228999107,
          17761989095418937083,
          14422989821906094960,
          17862722832973964554,
          17342128457906622114,
          2781203080110950796,
          10739817970598781322,
          4446779125771970845,
          12285095050143181504,
          18217340650421074707,
          13457249839261806890
        ],
        [
          3017354092126743884,
          6171346242030719614,
          7528569577771848187,
          17878279390517080526,
          15365337258079514864,
          17920113610663103521,
          5955249022964193120,
          16848488467412687849,
          14536860349507371483,
          3479756839920598107,
          3730227734749206308,
          184517833830328195
        ],
        [
          6333802488359288564,
          17165836944753806253,
          5414790557976932721,
          10028229613076045654,
          17520958275831821313,
          3332238079926646058,
          6653822311442449611,
          18038957787138027404,
          4627275314702597470,
          14324922090092204311,
          16578599978823080805,
          15195326001453433388
        ],
        [
          385832499433911170,
          8181554579313428477,
          5710040547836397287,
          4501673243763551551,
          18165813678900320098,
          3248663179473631726,
          13280842737265670609,
          8220589454662301084,
          1212388092925354426,
          11584732641804394589,
          16690563757925642782,
          9751755279299362694
        ],
        [
          16275518046409217250,
          3761068227668224653,
          2755650421432247163,
          1201382756939458237,
          14596475510415198307,
          1751088816857420356,
          12337874318566203269,
          17127329668058576616,
          17595396394663539720,
          6135810903355400982,
          11246498741460454117,
          7776427380305379772
        ],
        [
          3059627441225549092,
          277421853934539400,
          11648084688803875654,
          15189691095218716414,
          16669824014082445857,
          11613551740748377469,
          1896389549992581092,
          12465178787409318087,
          12625394734549848065,
          8019751923077039942,
          14729372926118477739,
          417415725784299428
        ],
        [
          105667853084182909,
          10123816103970937376,
          9279083338456394655,
          2302550908556856128,
          4930126858980535170,
          3425353102412358599,
          10670423343480326172,
          8983186942939892450,
          5773788538436885999,
          9141842036388690851,
          1308406012720414999,
          3740942581014422831
        ],
        [
          16099170655780048425,
          6947168519068662139,
          4805751774733295810,
          119576399473487713,
          9366402785251818687,
          11572144267541343103,
          13070543697477363518,
          11254712826515341154,
          7303298735180780200,
          18392609315587140622,
          11476529118317200021,
          3254434953756967763
        ],
        [
          4910638585813327331,
          16782320399580810208,
          15270005292318030649,
          14392734656089885833,
          7319186548244253319,
          9222412477794591047,
          3081719306580777694,
          5876244148384332636,
          2103806797650200026,
          6290669508892733263,
          856346681460851785,
          2878054206782990220
        ],
        [
          14063105711226278203,
          13260783137094841793,
          15633840147632623060,
          5364350656644986322,
          16162221879084843843,
          837706604867073305,
          4417788595758824968,
          10307755363167592709,
          8178817360078691253,
          10400714484597850205,
          2977610501794915325,
          15425191743983671429
        ],
        [
          11050941913431756275,
          4056813213298834829,
          11439644978095286391,
          146675048956631711,
          1765961373613654756,
          742892786477896698,
          15574277602545014264,
          8139014841095552697,
          14701769286808142936,
          9311597826198045590,
          7438180002684280974,
          8211330771617064737
        ],
        [
          15769894319565152022,
          13022673275080627231,
          12890046339494545101,
          5761899564425681584,
          4977456941905415160,
          4295745672480531263,
          7574658082311678735,
          4704629211441627450,
          7533647681067835287,
          14464987733466507177,
          8358775635941053695,
          13701340913663080573
        ],
        [
          15378314767024789767,
          1328397042999476541,
          6773834227155132617,
          990955697647620420,
          2355670438106758672,
          4543066799715849787,
          6639097559908392787,
          18290362644025643829,
          11705008670128077280,
          12472926550533959777,
          17346183397625007231,
          2409906215577380955
        ],
        [
          8284266277289441233,
          15415949049814533406,
          2990340796980121783,
          3818786372911688695,
          18122923382010139184,
          18204579264109270689,
          2026977682003213366,
          17538888205758573255,
          2482669119675747717,
          9086290179392586616,
          17129014728748691933,
          1170022956271866653
        ]
      ],
      "reference_13_bids_us": 29839.208,
      "reference_calls": {
        "30": 1060,
        "31": 873,
        "32": 873,
        "33": 873,
        "34": 873,
        "35": 992,
        "36": 611,
        "37": 611,
        "38": 611,
        "39": 611,
        "40": 611,
        "41": 545,
        "42": 3
      },
      "universal_route_us": 1227.542,
      "universal_reduce_13_bids_us": 7385.541,
      "adaptive_compile_us": 6699.625,
      "adaptive_route_us": 67.625,
      "adaptive_reduce_13_bids_us": 5940.542,
      "adaptive_statistics": {
        "nodes": 1908,
        "edges": 1907,
        "leaves": 233,
        "support_incidences": 2279,
        "python_object_bytes": 1472147,
        "max_depth": 12
      },
      "reference_bid30_us": 3405.291,
      "universal_reduce_bid30_us": 627.541,
      "adaptive_reduce_bid30_us": 481.959,
      "full_reference_bid30_us": 5413.458,
      "full_reference_bid30_calls": 1907,
      "values": {
        "30": {
          "5": 32,
          "12": 16,
          "17": 33
        },
        "31": {
          "5": 34,
          "12": 16,
          "17": 33
        },
        "32": {
          "5": 34,
          "12": 16,
          "17": 33
        },
        "33": {
          "5": 34,
          "12": 16,
          "17": 33
        },
        "34": {
          "5": 34,
          "12": 16,
          "17": 33
        },
        "35": {
          "5": 34,
          "12": 23,
          "17": 33
        },
        "36": {
          "5": 38,
          "12": 28,
          "17": 36
        },
        "37": {
          "5": 38,
          "12": 28,
          "17": 36
        },
        "38": {
          "5": 38,
          "12": 28,
          "17": 36
        },
        "39": {
          "5": 38,
          "12": 28,
          "17": 36
        },
        "40": {
          "5": 38,
          "12": 28,
          "17": 36
        },
        "41": {
          "5": 39,
          "12": 33,
          "17": 37
        },
        "42": {
          "5": 40,
          "12": 40,
          "17": 40
        }
      },
      "all_values_equal": true
    },
    {
      "seed": 790003,
      "tape": [
        [
          14338745003568726328,
          2243899417265048854,
          8169422533860627369,
          11692917138306285814,
          14283509374999852885,
          4734938437131273435,
          10618880391902369517,
          7214114786291611762,
          836016465147317005,
          13090563604068696387,
          1242187980223575526,
          7180982909638875922
        ],
        [
          13945730933790054010,
          6554552324733155759,
          14693010922504516403,
          10181730580106016907,
          5766662960533087479,
          1756378704808037684,
          3108888214202319039,
          7506904661363778560,
          7928345048334851714,
          16620871097828820536,
          7946335832003480362,
          13001813342648242524
        ],
        [
          8042135308111789602,
          15296206944642496593,
          5914930602730345073,
          6717495500604626251,
          4902955877055601754,
          3250742513372612105,
          2383033363794766469,
          11297963749205749159,
          15692233092686595585,
          2619353687579602638,
          15988759055531373677,
          11557371314856309240
        ],
        [
          14313650922523233182,
          848206680371376563,
          14572426360348161789,
          17288931865799645714,
          5075411319772366381,
          5974713381419174853,
          8980214645474967634,
          17830715726223507821,
          9173954188540760682,
          8072021836047070442,
          2154813486482050528,
          7460762172999015794
        ],
        [
          13573774696170799951,
          12268416739511984148,
          6607611081592108137,
          6963460636220208387,
          3271267019402428354,
          11697928828355686121,
          15401811548360112303,
          8606600740749160400,
          87137060987813196,
          14568228713626436917,
          11253036538126060831,
          9456142833208336145
        ],
        [
          14777157777398359151,
          14179153293268319288,
          14419666361745764674,
          2186879708099792917,
          18221129752697846690,
          13440889507500761818,
          12241114768479218827,
          2752192235102844140,
          11554818128466817061,
          2823808637214252898,
          8009446251461069454,
          6095068185028366033
        ],
        [
          7508663690381446676,
          4580037087225345166,
          8522549534046806751,
          3915428675494305071,
          9646837789921547052,
          6039228367390720247,
          7374131109019654416,
          5489117420713893819,
          640081557797101377,
          7369132386753353618,
          8909835835408743355,
          1749213937779004908
        ],
        [
          17081973684039584688,
          2372529068612859461,
          16052796749325472779,
          14874943362358724505,
          8616311676452009080,
          10988369421744329571,
          4462309489732759638,
          17724900588113533831,
          1757778129517247546,
          12544885930455562810,
          8032127088531708271,
          18008244966590163767
        ],
        [
          12277918697471517004,
          5484155087404685356,
          5512080856648519551,
          12135468110006064244,
          8661367138175035082,
          6769835365278133433,
          17187765186526930578,
          4558929007051057820,
          9504070377543271826,
          15943489700461249933,
          3065580653760052380,
          15821761804763402598
        ],
        [
          8693799832886558301,
          15496085673420931490,
          17908594035397372133,
          8458797057940679163,
          5176518402526982986,
          1042010627971156482,
          18175830596344648378,
          6942113391652827255,
          1753156515691066768,
          9058856664432865641,
          18244957583008018392,
          8946777408385451627
        ],
        [
          15232662232323293908,
          10025270159956523877,
          13801012093664406594,
          5763580478276580616,
          2396375283688227389,
          9748352722351775702,
          11777857558260794784,
          7036894554053686447,
          2127284053526644654,
          2658619600241613925,
          910099167747701232,
          5774100061070415952
        ],
        [
          9002741388053274888,
          1805366854889945209,
          4942375049368321619,
          7768342150781765456,
          5477274784517736733,
          15795699409621046717,
          7396189038538304122,
          17694718971878176440,
          12887027611244499541,
          9430670170013527090,
          11013713107933925398,
          5708505737236876658
        ],
        [
          3210551685481329998,
          7866010672240212358,
          17065251686958359389,
          12742183643817926799,
          17752186163884478603,
          8000505563593103956,
          420543253758312145,
          15757982002699648707,
          12868464940827831516,
          18060617048339974890,
          3071459570973175039,
          5061646080191734260
        ],
        [
          13012310310981373685,
          18372533475438849884,
          1059296922508965723,
          17648130568702563780,
          8791241032870392884,
          2767469470108536942,
          17312381011498943568,
          6208693455637918873,
          6803126599784043383,
          11106686371715742135,
          15693756003802866446,
          16004999702093523344
        ],
        [
          5803038049982419803,
          809915557965415675,
          16822157668675618613,
          16125676315201081479,
          2977845645907062634,
          7380361746367027982,
          3596942532557879367,
          11606337761556695544,
          7450990342012572755,
          9253531106657299050,
          8846281418181369382,
          11341373723399677037
        ],
        [
          8252692480389938396,
          429437707898440815,
          3239454354283620919,
          17644171063043316808,
          10006238269343899139,
          5871634845906504280,
          13699560232924173406,
          13782414018081663966,
          13532970506565961492,
          4998322611008728543,
          17481618973432287847,
          12676565328445275541
        ],
        [
          6434784812366327331,
          2284758587793050240,
          11337458001046548372,
          5602578028359971477,
          15911595131322202654,
          8868045902072148073,
          1216489036760403062,
          15544412430825300067,
          4237668752483839232,
          13524730394755579076,
          14837619408850443293,
          3876522645140104108
        ],
        [
          10866153036037790125,
          15518438860073196822,
          229223545212686430,
          15974110063516418783,
          15101367134798313851,
          6255088840285732955,
          1773391310719911291,
          7602735840795761142,
          6170397268022776194,
          7515896723910167094,
          16961804099764605513,
          9107102709705426273
        ],
        [
          5399074347598375098,
          10018791937845822526,
          9923637847581308707,
          11181445872112362632,
          16607331321984674425,
          7233733601394249513,
          4862980633219094856,
          7789370244362388548,
          4603700417646378756,
          15774278182098718941,
          7392864977235003556,
          1627076291538476861
        ],
        [
          11895257354053552977,
          11101481975664879742,
          18409447372481308301,
          7000702355996285846,
          17132121616026431457,
          2168241630358014769,
          8966301915818763462,
          16152446747016901518,
          6974283707924235831,
          15168877159878968617,
          14346579289118591689,
          678955145849768504
        ],
        [
          3234785020307453051,
          1184344429250264241,
          14965528606840908066,
          13717306472313607950,
          12347499334322006216,
          13681940262392615140,
          2678195428563320854,
          9719767585448277471,
          3844258943748595739,
          13982844130369814425,
          7559569939880693650,
          17220433914472544112
        ],
        [
          12946341632689815451,
          15303819218860771936,
          9741002277880226985,
          2727761088187855561,
          14792002194571789061,
          11823626295679789086,
          15224863844529648041,
          17765401503736351129,
          4880341901320878276,
          6543842727330957209,
          12307285634161407847,
          4878986062609127505
        ],
        [
          3399389150909825958,
          8029994769242510151,
          13432038071549239479,
          4033552780208302216,
          46401492007039591,
          13454845569981343641,
          13111594720685381629,
          10154074904837485389,
          6806765710207846495,
          8088940472019801504,
          15038550700671181689,
          3769469115743173476
        ],
        [
          7083734100844748633,
          11980030775231421639,
          11033439787439339807,
          8330265355682189518,
          2521863983058621913,
          3862401811448610581,
          5937372648892217426,
          13852020602536908143,
          6990878994762752176,
          9323291943862934354,
          12048345453379309008,
          8977545535904991628
        ],
        [
          6784666145119972180,
          7155929187312400571,
          8881770279407267706,
          12752456739670299132,
          1981914135577253229,
          2271241259386810392,
          3261179445015117980,
          13403571712013531637,
          8145497929314382228,
          8376944815781557190,
          10431793676435469742,
          7415808810846971170
        ],
        [
          2749597944911520413,
          10960064577175436804,
          11116888778231430654,
          8221983106157917312,
          14162966509294562843,
          16050278552425784416,
          12910115092588149332,
          15154237015283909123,
          9816104977361855516,
          11854618491350241422,
          9846825723285573090,
          6115972481587570372
        ],
        [
          7125868374979972491,
          8091416784177158661,
          15779708263638637257,
          9370656093062779151,
          15972691618390994502,
          4316386717443535653,
          6277052001912431583,
          3132481977987005792,
          14758276945710792470,
          281777375268933342,
          724855905615269247,
          15945410017182806554
        ],
        [
          7692538817677768840,
          7892764788050715673,
          10061996358209819862,
          2445809697228769958,
          7129153759278374496,
          15635436943581750702,
          7109107708323997457,
          1500013875552850357,
          5065849827509300612,
          10390354640054084473,
          7581406998447796260,
          1486074540246375002
        ],
        [
          9490294998636131039,
          16138529256370733608,
          8838821064136172120,
          12536378921421155003,
          13855826137813958949,
          1619083220785244588,
          1717526218898581768,
          8717534855823160211,
          11658606925718770698,
          16761204785645486368,
          6266661290757374163,
          6184065512432944865
        ],
        [
          1748557070685196202,
          13526129408745023287,
          11730343523158024993,
          2658521636189290674,
          14942953707647665246,
          2703210845059221543,
          5181046834592179592,
          5608105182227382870,
          7471165130468143517,
          16942542380583613681,
          16056113913987773558,
          6756842698889573016
        ],
        [
          16662010961388874079,
          16077321940200042017,
          2574068612894590498,
          10890720931852694152,
          3165586953171988481,
          12998351631818945906,
          4160406204501650091,
          475081520960867501,
          6371058484287820999,
          14300928178027998390,
          14328095326125684012,
          4339308221898482609
        ],
        [
          572102575938842393,
          3571409015484311288,
          1158157151567542842,
          17186415098943241806,
          8301635198665111123,
          225828204936686216,
          6248412691317596624,
          10575207159344403768,
          15287248947561152596,
          16556805825904398092,
          7887349098305293932,
          10860884481417244409
        ],
        [
          7070612839913697191,
          10569622239754875849,
          1903178911774124932,
          15111297542924171084,
          4169528357616903664,
          7494737290484263759,
          12238177794937759793,
          10025387672497147314,
          11238878362914800102,
          4809736595713310872,
          15654930471556053113,
          16366233217316686629
        ],
        [
          28838594296468982,
          972738375042524159,
          16286751816855371933,
          5712541922315559907,
          1274714692977636797,
          3426621870126774350,
          9390580750002143520,
          4261617121571149155,
          15229457636600935153,
          4266223976089455554,
          15775937417669178327,
          10218332268250683894
        ],
        [
          15291060593213584678,
          15657383498816499200,
          11114070141767227489,
          9490607399911718098,
          6394905409230260728,
          12797503276203525357,
          9456727218099861649,
          17561415573036630483,
          2002458666245745521,
          13580592463781756939,
          15511019338831115923,
          18312787840380836072
        ],
        [
          9099516105028236905,
          16758786966330023423,
          13568529258043968472,
          5024974546621813316,
          1871415736875998819,
          9492294809740024960,
          2899701774189735334,
          17779927783405432187,
          3829845821897756883,
          10672522066540109972,
          10051126984244033678,
          15651787425650903168
        ],
        [
          1094543935025161833,
          11218280140845264140,
          15547367655301995560,
          4899877833805918668,
          11453711942743802886,
          2036846413073949663,
          15324530428221892321,
          4193349652454397204,
          1896931619485927314,
          10658781334310120352,
          9339667764520273666,
          836031709670450999
        ],
        [
          342923079466983689,
          3645150742093933538,
          506648946505941306,
          17654634347562421046,
          5433001124719854407,
          1938416679683321425,
          6145006416316636712,
          4313843251904091227,
          505923042438966951,
          11423103886129020958,
          3898665646517518593,
          2749607688631712221
        ],
        [
          9458550820798737889,
          10725290561008364450,
          18413899615741916571,
          14400929023813323078,
          7111827871097383421,
          9864957414565730962,
          5569547002176918542,
          10748822778727964486,
          14203524630675748099,
          3487375558632648467,
          18228073287279123372,
          5589577165682093685
        ],
        [
          5749647978536674349,
          14775967288035181556,
          15095232563400286599,
          3635939334913366236,
          2808238635454953307,
          15070325707030363209,
          17652264237043986180,
          12088863888848981579,
          17656962497021859197,
          17155877134466882948,
          15007206280391336085,
          13687912316250648737
        ]
      ],
      "reference_13_bids_us": 26802.875,
      "reference_calls": {
        "30": 918,
        "31": 763,
        "32": 763,
        "33": 763,
        "34": 763,
        "35": 978,
        "36": 498,
        "37": 498,
        "38": 498,
        "39": 498,
        "40": 505,
        "41": 522,
        "42": 3
      },
      "universal_route_us": 1391.292,
      "universal_reduce_13_bids_us": 7612.291,
      "adaptive_compile_us": 7022.0,
      "adaptive_route_us": 65.084,
      "adaptive_reduce_13_bids_us": 5744.25,
      "adaptive_statistics": {
        "nodes": 1875,
        "edges": 1874,
        "leaves": 232,
        "support_incidences": 2287,
        "python_object_bytes": 1448851,
        "max_depth": 12
      },
      "reference_bid30_us": 3010.125,
      "universal_reduce_bid30_us": 727.5,
      "adaptive_reduce_bid30_us": 467.5,
      "full_reference_bid30_us": 5365.416,
      "full_reference_bid30_calls": 1874,
      "values": {
        "30": {
          "5": 30,
          "12": 13,
          "17": 28
        },
        "31": {
          "5": 31,
          "12": 15,
          "17": 30
        },
        "32": {
          "5": 31,
          "12": 15,
          "17": 30
        },
        "33": {
          "5": 31,
          "12": 15,
          "17": 30
        },
        "34": {
          "5": 31,
          "12": 15,
          "17": 30
        },
        "35": {
          "5": 32,
          "12": 21,
          "17": 33
        },
        "36": {
          "5": 38,
          "12": 27,
          "17": 36
        },
        "37": {
          "5": 38,
          "12": 27,
          "17": 36
        },
        "38": {
          "5": 38,
          "12": 27,
          "17": 36
        },
        "39": {
          "5": 38,
          "12": 27,
          "17": 36
        },
        "40": {
          "5": 38,
          "12": 27,
          "17": 36
        },
        "41": {
          "5": 39,
          "12": 34,
          "17": 39
        },
        "42": {
          "5": 40,
          "12": 40,
          "17": 40
        }
      },
      "all_values_equal": true
    },
    {
      "seed": 790004,
      "tape": [
        [
          3461855727266562682,
          11237330571808342570,
          11929214108797838515,
          4918096528714513380,
          3505660265276468117,
          1929567908206406842,
          5775710089808503086,
          14976508820375975927,
          4573456922424645194,
          1524039175252459980,
          3048525298828860669,
          6032573946958001684
        ],
        [
          6866106855074814738,
          6313767339511879139,
          10114097757045532703,
          12057551704089963379,
          7700770895009348170,
          18259592683230280767,
          4454597142657030164,
          17864953336709148380,
          8520887721704591473,
          9779423799869931269,
          11112873789896189426,
          17017636773717725715
        ],
        [
          1567295465787743557,
          12821936875251763294,
          1964326041934917401,
          5913099380584599477,
          16483463255907967945,
          1547220861071263235,
          8712527590779604468,
          11747809994225017797,
          7616114244095927352,
          2290893653516051101,
          9136347817331699625,
          17658534296530040074
        ],
        [
          8595601901549252721,
          5235428023851387925,
          10330676906488254025,
          68553400298952736,
          2009863260395383651,
          9458693826198549344,
          9452723304412620818,
          17762650855240056164,
          2282816068969851837,
          12328809548963712,
          15595696021241892822,
          15872935611568653877
        ],
        [
          2780591193623823042,
          9552140682779999889,
          13923764440290070046,
          2493163886163196406,
          3237117703434767554,
          15108481314147026034,
          2616157220094605705,
          9302561383114343210,
          15373482201617682081,
          15851005504319017087,
          5553170302977631700,
          6983967089415230300
        ],
        [
          14606015125178417921,
          3200430309707550085,
          16963613109688736613,
          16909700549544586329,
          3393290419856335310,
          4444785674041858484,
          10642130019895911016,
          11155253930330383474,
          4373289068526886587,
          13898795010874855656,
          7760329165313818910,
          1136670310624204743
        ],
        [
          4007690723910547541,
          12363885322982801489,
          14901676576924605533,
          2273531498256323534,
          4597577561768820958,
          5104335235526489248,
          2130216146537421473,
          6437793968035951278,
          8354616228273832247,
          1568594871972033875,
          16774059760029614044,
          4263595561069512166
        ],
        [
          9214843263309130151,
          2686672363562953891,
          11620302817331049746,
          3189342471302404443,
          9268460363212485630,
          13408731141966318294,
          18388654899006766010,
          17856285836633971834,
          14906321265945919825,
          2002386260943761059,
          17819610831494089348,
          8657005914411120797
        ],
        [
          4847862013539789580,
          11404710678538795776,
          4570628685135365706,
          7182554587029321144,
          7168972426209693107,
          7085103057971571438,
          13630964059732130492,
          5975908841233931340,
          15205043392557113107,
          1339614766591713531,
          3635313105289518598,
          8224529515170031693
        ],
        [
          1568264023665698918,
          18205783570574999781,
          17888913938466610324,
          2991621924552278135,
          8727855134203081960,
          5781824464251308415,
          10503392205438186132,
          17655483209488729170,
          4631100860311001779,
          6055089468659728209,
          15802462859475629058,
          5180310125881748409
        ],
        [
          8972027412702322815,
          18364912200448855439,
          5033926975941504735,
          4441277435912211824,
          5298397123256528526,
          379840545537041530,
          8043617645586265880,
          1797964294566970370,
          17547579499340621554,
          3160029399141920989,
          2071684566951195260,
          17101083725608121764
        ],
        [
          7181998499487391379,
          5797279671878321025,
          145806786462107807,
          16831682507899020716,
          9091842243714432056,
          16251433184550701764,
          3264976843042781176,
          14747920152270128510,
          7357276870140100011,
          14973911824932328490,
          7778016948330023344,
          5035529724961555218
        ],
        [
          9220654749703370719,
          3864834487056633207,
          8619289385550529031,
          1418307647020208016,
          8565860086704039369,
          11119280410104666026,
          7265379221984647901,
          7265986867287256902,
          11762567239399959565,
          4159188204438663657,
          14128880831805772237,
          5054539738794003381
        ],
        [
          9698905752791424023,
          7090962181792934844,
          13066297942380211886,
          3159273528742223754,
          12526907225178491023,
          15987671402424257003,
          1789699290908519537,
          206467124868751015,
          13345833797977174904,
          13719000183502785809,
          12336717770751087540,
          5536346291730702834
        ],
        [
          13458188417341351394,
          400198228731742301,
          8924053194134465222,
          8476058947526861637,
          13808081790296648319,
          1147097905287025739,
          8624687503220528183,
          17609583189227018776,
          2916957151507228338,
          3865958474158644909,
          6606542462543367183,
          9159478130991525154
        ],
        [
          10391030142729105716,
          4266766314779522177,
          7642967303534440323,
          16971348062732913738,
          13476317528788138011,
          309158339762786993,
          15438184326883889875,
          12021839528549409066,
          11428143881661415394,
          12496287959546543266,
          9287134082727773081,
          17401325804322568786
        ],
        [
          8781355005562076783,
          16707556957421411997,
          3794910787298064983,
          17291276066196120688,
          11602873713041837990,
          14102262271351684579,
          12418244425174557836,
          15338381880622045022,
          5707294150081132836,
          8072823735913603526,
          14758956613282894413,
          14755290344128379381
        ],
        [
          17414865837004493817,
          7584852678649903186,
          13942720331414954917,
          2384345611257657502,
          13890884224281506358,
          11685293647439903552,
          6626284112643283418,
          16924471933502572917,
          15248592537115606601,
          13487568089292094087,
          1341192014085664847,
          5579599197170946409
        ],
        [
          4114554006972410041,
          7060487316826798199,
          14297320004667014362,
          5865257490157117533,
          1437683597213628327,
          6573761817926445520,
          5662502868958717824,
          5928554933367516445,
          2459935494086489188,
          73259498263003610,
          152607682715700997,
          3676535278123186533
        ],
        [
          2242058199048494360,
          13598708384807633383,
          16609727094398328421,
          6011831728560447650,
          11271085121284073062,
          4427209077813306627,
          666873120355103945,
          344711675917797217,
          3054198537449854580,
          6648197051567461672,
          2587453311136392324,
          9191329988170842997
        ],
        [
          1338602218129016376,
          17600116712028066275,
          17970985159130838322,
          6147783888957302712,
          6164899781376951059,
          4842362594484945830,
          1890068120519761564,
          7045464426792152670,
          13791614609214993658,
          12570131137634496186,
          6277193070905780031,
          18410606950703517421
        ],
        [
          7074990357693280369,
          3982629808088827456,
          16892048162081003748,
          11138603332405927341,
          11819643752037349141,
          15728511200189698037,
          8432163026408232217,
          2151103137031704118,
          7030976907254325683,
          13536588561861623548,
          4992474325896154088,
          5693106673944050115
        ],
        [
          3371675784459730362,
          1413299937324131163,
          11292459598329701172,
          4266839434599335290,
          2253793894715488509,
          3803495091569459233,
          1651845320354295663,
          17656287688237507873,
          4401854497698186860,
          12635834162181422719,
          2234642348414555506,
          11069796160307291267
        ],
        [
          16156924816833572281,
          15190756447821508302,
          8211308821516387903,
          12202314616093588475,
          6354068211055899144,
          8272814926874379529,
          11663531337366155184,
          1793875381892137285,
          6694061556224779265,
          9192299074374874570,
          10601804560048259695,
          11419028242094599865
        ],
        [
          12359746336008567964,
          14549408238757516645,
          11549155138026224754,
          8745272308919064244,
          1848879215370805264,
          17037737410077059353,
          10705640404940347407,
          8991418231309809043,
          5379725087179799620,
          5277772744568092308,
          15006767734918417383,
          1032093528812840878
        ],
        [
          3083473381022600273,
          10627030895876472580,
          644349335625155669,
          4775714854239317821,
          4864855280034128610,
          15179713850043970217,
          15167499531341902708,
          9041938656803617905,
          2445345757784777693,
          8274953881823137968,
          8896533733720646160,
          1647401586235979067
        ],
        [
          2353499762288267565,
          14868573935191285671,
          13910274557524445221,
          17812684487628768752,
          4302439823905943322,
          12996716217111317238,
          2446516426497207662,
          3521463909063029836,
          16651367594982243584,
          15720923615918621668,
          6918750924795459347,
          4448979312512103630
        ],
        [
          11282406245896485162,
          908247407611172868,
          9573285733778541976,
          12580594979507923532,
          5415392787618695007,
          2373535056093275489,
          15088878176250722998,
          3263911052628927523,
          9287458211185485105,
          4415835516123300198,
          3262935042552867118,
          13168082160656351025
        ],
        [
          15251921563915128034,
          9455628393914934625,
          1535774919053158395,
          7306652221397041739,
          2789718288554409829,
          7904544688199836348,
          6124847173891045077,
          6183360358205891146,
          3846457422508001226,
          3725997026188932256,
          7583358372539310059,
          4285139811733690862
        ],
        [
          757365719064352801,
          2770845797413579679,
          17451965866327636936,
          4200396604017945932,
          4142726137259436911,
          11135952038070817381,
          8993929421504420973,
          1086927180692876478,
          7242161961266299757,
          1506033914249722322,
          10628214331673061096,
          8211543470742764586
        ],
        [
          4542083664276097740,
          10650663912611864579,
          7659839943448174791,
          4024253221953606192,
          7037513045535782187,
          12787124547607787903,
          13577137634733350427,
          13226339763916147156,
          10312860662184296022,
          6099401316800587131,
          87845673211519730,
          6190390289596192068
        ],
        [
          11047312839106770884,
          8301974384292207136,
          16867387149285098921,
          17907331728144692350,
          10436168343747590278,
          18147632333910407019,
          6163707160435796283,
          8639464653191165020,
          4590080597777072435,
          1900022226356080550,
          9283511668077296680,
          8939217151826308563
        ],
        [
          9416134553548795207,
          8186725391658897063,
          17056445388849742006,
          6850244528303941373,
          4866139121387737426,
          15629302148484827584,
          16014092576900858062,
          9317216454332314662,
          13596546523773147688,
          9419805096121551136,
          7080659672851894728,
          7366878491601206232
        ],
        [
          9665986361662126095,
          7089402412727534287,
          8717335695395357326,
          2319491407204826024,
          8574119609237469840,
          17100783421312487242,
          4212799257377247571,
          5646762036117765413,
          8090723230076081051,
          18004466938018388052,
          16728392993298016993,
          4187279759749970738
        ],
        [
          2015055334986596214,
          17296902524216938512,
          17666680926565443959,
          3925036429358532121,
          12020158527526587958,
          7428615857891150348,
          5538338560119332959,
          14568454484244097290,
          9076953619047400233,
          18355366733815741203,
          12084188670018968292,
          3768628490473278970
        ],
        [
          13827292076280912427,
          12982632115466971233,
          14960943050099297097,
          12669902673376351617,
          9481909872942517557,
          17561458263067472151,
          747699641537527460,
          12271414034652792370,
          16295533921418548111,
          4137710439577774743,
          11956045421896095496,
          7014417168069232492
        ],
        [
          12269951109475645696,
          8500025886472087542,
          8079578085043959901,
          2530569966487736348,
          2064179480148396224,
          15889943942422197269,
          666737502364927492,
          40495607009027925,
          14261754109750743176,
          15422895911055758209,
          5932666041565378660,
          13497485332421083668
        ],
        [
          15821701744151022452,
          3433156321340436400,
          15779400803875442535,
          7193444231582854892,
          10095735409842603417,
          6225356987963526769,
          17176034553976880352,
          13947371103743012072,
          2860420408361444209,
          11625805528549993425,
          4730191703823235960,
          8747119043805740760
        ],
        [
          10278243494715766462,
          4524336567542192367,
          8758847960734239242,
          379413998005804172,
          17977292172830650056,
          17161517377448115577,
          12477484803071824014,
          7366556443091311174,
          9981435474665498534,
          3142122832448562223,
          4053308302775325256,
          14478595739554764641
        ],
        [
          8370627886242694250,
          12922911418088052280,
          9865798058090197193,
          9521928801687673050,
          6255112115327317190,
          4259391269647480601,
          10489990003633258784,
          10448128843885756595,
          3843048997944469444,
          14912801463638486394,
          18399796753025944382,
          910205692843671889
        ]
      ],
      "reference_13_bids_us": 26560.75,
      "reference_calls": {
        "30": 1005,
        "31": 743,
        "32": 743,
        "33": 743,
        "34": 743,
        "35": 909,
        "36": 523,
        "37": 523,
        "38": 523,
        "39": 523,
        "40": 523,
        "41": 451,
        "42": 3
      },
      "universal_route_us": 1384.667,
      "universal_reduce_13_bids_us": 7582.792,
      "adaptive_compile_us": 6932.792,
      "adaptive_route_us": 55.167,
      "adaptive_reduce_13_bids_us": 6007.458,
      "adaptive_statistics": {
        "nodes": 1935,
        "edges": 1934,
        "leaves": 234,
        "support_incidences": 2324,
        "python_object_bytes": 1493899,
        "max_depth": 12
      },
      "reference_bid30_us": 3282.667,
      "universal_reduce_bid30_us": 599.0,
      "adaptive_reduce_bid30_us": 471.417,
      "full_reference_bid30_us": 5480.417,
      "full_reference_bid30_calls": 1934,
      "values": {
        "30": {
          "5": 36,
          "12": 17,
          "17": 35
        },
        "31": {
          "5": 36,
          "12": 18,
          "17": 37
        },
        "32": {
          "5": 36,
          "12": 18,
          "17": 37
        },
        "33": {
          "5": 36,
          "12": 18,
          "17": 37
        },
        "34": {
          "5": 36,
          "12": 18,
          "17": 37
        },
        "35": {
          "5": 36,
          "12": 22,
          "17": 38
        },
        "36": {
          "5": 39,
          "12": 31,
          "17": 38
        },
        "37": {
          "5": 39,
          "12": 31,
          "17": 38
        },
        "38": {
          "5": 39,
          "12": 31,
          "17": 38
        },
        "39": {
          "5": 39,
          "12": 31,
          "17": 38
        },
        "40": {
          "5": 39,
          "12": 31,
          "17": 38
        },
        "41": {
          "5": 39,
          "12": 34,
          "17": 38
        },
        "42": {
          "5": 40,
          "12": 40,
          "17": 40
        }
      },
      "all_values_equal": true
    }
  ],
  "universal_compile_us": 312536.0,
  "universal_statistics": {
    "nodes": 86988,
    "edges": 86987,
    "leaves": 14871,
    "support_incidences": 95989,
    "python_object_bytes": 64894815,
    "max_depth": 12
  },
  "coverage": {
    "union_coordinates": 6327,
    "union_support_incidences": 7317,
    "tape_coordinate_visits": 7585,
    "tape_support_visits": 9139,
    "unused_coordinates": 80661,
    "depth_counts": {
      "0": {
        "physical": 1,
        "used": 1
      },
      "1": {
        "physical": 3,
        "used": 3
      },
      "2": {
        "physical": 27,
        "used": 27
      },
      "3": {
        "physical": 143,
        "used": 112
      },
      "4": {
        "physical": 647,
        "used": 253
      },
      "5": {
        "physical": 1566,
        "used": 429
      },
      "6": {
        "physical": 4025,
        "used": 595
      },
      "7": {
        "physical": 7800,
        "used": 730
      },
      "8": {
        "physical": 13870,
        "used": 823
      },
      "9": {
        "physical": 14375,
        "used": 837
      },
      "10": {
        "physical": 14789,
        "used": 839
      },
      "11": {
        "physical": 14871,
        "used": 839
      },
      "12": {
        "physical": 14871,
        "used": 839
      }
    }
  },
  "status": "completed",
  "attempt_seconds": 0.9632225000000001
}

````
