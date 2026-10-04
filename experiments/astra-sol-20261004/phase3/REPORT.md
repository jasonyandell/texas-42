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
