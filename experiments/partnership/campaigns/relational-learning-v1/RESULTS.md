# Shared Scheme learning: implemented, measured, and still behind sampled tables

2026-09-07. Exploratory finite-domain evidence; bid 30, make/set only.
[Machine-readable measurements](measurements.json) retain the configurations,
root groups, selected programs, exact fractions and construction diagnostics.
The [instrument guide](../../RELATIONAL-LEARNING.md) defines the target and
reproduction protocol. The proposal is preserved unchanged in packet commit
`14e01322`.

## What is now executable

The packet's initial native path is implemented: decision provenance and
controller traces; a bounded shared relational actor; action-cost teaching;
on-policy data aggregation; whole-policy development selection; a separate
correctly centered information-price examiner; frozen deployment programs;
matched frozen-table fallback substitutions; and interruptible parallel jobs.

The learner searches ordered lists from fourteen versioned mechanical clauses,
with at most three clauses, 256 AST nodes and a six-candidate beam in this run.
An empty baseline is always retained. Its actor has one mode, no rigid bindings,
no exact keys, and no lookup by source deal, original hand or history. Different
hands receive the **same program**, whose selectors can return different tiles.
Actor predicates receive own/public information. Hidden Scheme events and exact
worlds stay in the examiner.

This implements the first bounded, stateless learning loop, not every future
extension discussed in the proposal. There is no learned opening policy,
score/history predicate extension, mode induction, coefficient learning,
belief-summary actor input, adaptive teacher allocation or price-based pruning.
The existing L1/L2 player is unchanged.

## Frozen experimental design and actual scope

Each of two separate target panels used 144 source-deal groups: 32 initial
training roots, 32 fresh on-policy discovery roots, 16 development roots and
64 untouched final test roots. Root ranges and the grammar were fixed before
selection. No starting hand crossed a split. The test computations began only
after the selected program digests were pinned.

The focal player had **three dominoes**, was next to act, and had at least two
legal choices. Bid 30 was unresolved. The support cap was 512 worlds. Sixes
were declared and S0 was bidder/viewer. The legal fixture prefix and uniform
mechanical-root prior are explicitly described in the guide; the posterior is
then conditioned on the frozen field's observed actions without rebasing.
These are endgame continuation results, not opening-game make rates.

Every evaluated root used its whole compatible fiber. The 64 final roots had
11,836 worlds in the gym-field panel and 8,892 in L0-8. Those worlds do not count
as independent starting hands. The two fields and seed groups differ, so their
absolute make rates are not a head-to-head field comparison.

| Panel | Complete pipeline wall | Workers | Frozen actor size |
| --- | ---: | ---: | --- |
| Maintained L1-partner/L0-opponent gym field | 39.411 s | 10 | Three clauses; 829 bytes for the exact-regret actor, 707 for the interval-cost actor |
| Native L0-8 | 11.352 s | 8 | Development selected the empty 72-byte baseline for all three cost arms |

Whole-pipeline wall includes data generation, both discovery/fit/development
rounds, exact truth cross-checks, pricing, serialization and replay checks.

## Held-out full-policy results

All figures below are equal-root mean make probabilities. The table is fitted
to 16 sampled worlds at each root. Every hybrid retains that exact table's keys
and actions and changes only what happens after an unsupported history.
The table gets per-root online construction; the shared actor runs its frozen
rules without online search. This compares a deployable compiled baseline with
root-specific planning, rather than asserting equal computation between them.

| Frozen actor / composition | Gym field | L0-8 |
| --- | ---: | ---: |
| Sampled exact table, lowest-legal fallback | **46.197%** | **50.518%** |
| Shared actor trained on exact lawful action regret | 40.726% | 37.893% |
| Same table + exact-regret shared fallback | **46.967%** | 50.518% |
| Shared actor trained on continuation-cost intervals | 43.576% | 37.893% |
| Same table + interval-trained fallback | 46.228% | 50.518% |
| Exact lawful root optimum under that frozen field | 49.503% | 52.427% |

Priced and unpriced interval teaching produced identical selected programs.
The exact-regret shared actor trailed the sampled table by 5.47 percentage
points in the gym field. Its descriptive paired root-bootstrap 95% interval
was [-9.58, -1.76] points. The interval-trained actor trailed by 2.62 points,
interval [-4.52, -0.91]. In L0-8, development rejected all learned clauses;
the selected empty actor trailed the sampled table by 12.63 points.

The gym-field exact-regret fallback substitution gained **0.771 percentage
points**, interval **[-0.122, +1.909]**. It improved 11 roots, harmed nine,
and tied the rest. This is an uncertain positive signal, not an established
strength improvement. The other fallback substitution was effectively tied.
L0-8's empty learned fallback was exactly the existing fallback, so its hybrid
comparison must tie and did.

First-action regret and whole-policy regret remain distinct. In the gym field,
the sampled table's first-action regret was 0.693 points and its whole-policy
gap was 3.306 points. The exact-regret hybrid preserved first-action regret and
reduced the whole-policy gap to 2.536 points. The measured difference comes
from changed continuations, not improved root-action selection.

The table reached a first final fallback before resolution with mean probability
45.50%. The exact-regret hybrid reduced that to 24.83%. This is a coverage
diagnostic; the paired whole-policy comparison above measures the actual effect.
Simply conditioning outcomes on fallback occurrence would not establish that
causal effect.

## What prices told us

The examiner used four fixed Scheme events and one coefficient vector,
`[0,1,-1,1]`, frozen in advance. Centers used exact rational conditional means
at each full public history. The queried action was fixed before future choices
were priced. Complete relaxed maxima and the outer expectation were computed,
and every returned upper was checked against exact lawful Q.

This frozen basis/vector tightened **zero of 64,806 action bounds** across the
128 discovery roots. There was little room at the root: mean unpriced
upper-minus-Q width, first averaged over root actions and then roots, was
0.186 percentage points in the gym field and 0.378 in L0-8. Pricing left both
unchanged. These narrow endgames do not test the full price class or larger
hidden supports.

The unpriced bound passes used 692,994 and 469,593 charged units; the priced
passes used 2,856,972 and 1,932,754. This is about 4.1 times the bound-pass work,
excluding the common tree build. Both routes had the same total allowance after
charging that build. No teacher acceleration was demonstrated.

A structural limit also matters: with fixed continuation lowers, changing the
upper in `max_b U(I,b) - L(I,a)` adds the same constant to every action at a
state. Under valid `[0,1]` bounds, clipping cannot alter this. Thus tighter uppers
alone cannot rerank the programs in this matched-cost experiment. A new native
test verifies that invariance. Prices can improve certificates and potentially
guide future teacher work or pruning; a stronger actor does not follow merely
from a smaller upper bound.

## Existing partnership exam

After freezing the gym-field actors, they were applied to the 30 maintained
partnership/bid-making exercises, spanning 21 source deals and additional
seats/declarations. This is an outcome-selected diagnostic, not another
unbiased sample of the preceding distribution. All native teacher optima
matched the maintained exact answer keys. The exam took 2.181 seconds.

| Actor | Optimal first actions | Mean whole-policy gap |
| --- | ---: | ---: |
| Sampled table, 16 worlds | 25/30 | 1.299 percentage points |
| Shared exact-regret actor | 21/30 | 10.929 points |
| Table + shared exact-regret fallback | 25/30 | 1.457 points |
| Shared interval-cost actor | 0/30 | 31.129 points |
| Table + shared interval-cost fallback | 25/30 | 1.751 points |

The learned rules are therefore not a partnership solution. Their weak transfer
is useful counterexample material. No rules were revised after looking at this
exam or the final test groups.

## Validation and provenance

- 66 focused Rust tests pass, including legacy Scheme/dynamics/search gates,
  actor access and cost tests, price centering/parity tests, and wire-format tests.
- Seven new Python runner tests and twelve existing policy-runner tests pass.
  Strict focused native clippy passes without exemptions.
- The proposal's independent rational verifier passed its 8,192 generic policy
  checks. Those are mathematics checks, not native 42 learning results.
- The native evaluator checks the exact on-policy performance-difference
  identity and interval regret certificate, compares its teacher with the
  pre-existing exact search, and replays saved/parsed/compiled actor programs.
- Independent Python replay verified **8,694 retained full games and 25,935 focal
  traces** across both panels and the existing exam: turn order, legality, all
  28 dominoes, 42 points, empty final hands, made/set, provenance and controller
  fields. This audits sampled traces of the programs; policy values themselves
  integrate the complete declared root fibers.
- A real SIGINT/resume check against the final runner returned incomplete status
  75, left no native children, and completed all 36 jobs on resume. All six
  results durable at shutdown remained byte-for-byte unchanged.

The [operational receipt](verification.json) and static audit receipts accompany
this report. The measured runners are retained under `reference/`: review later
closed a reentrant-signal/spawn-registration interruption edge. The final runner
was verified independently after that fix. Native search, programs and measured
policy values were unchanged. Full legacy Rust CI remains waived for this session.

Raw jobs, lessons, candidate programs and traces remain under
`/Users/jason/data/texas-42/relational-gym-v1`, `relational-l0-v1`, and
`relational-exam-v1`. Six frozen selected actor files and compact per-root
evidence are checked in here. Reproduction uses the pinned recipe; resuming an
old measured job requires its original source identity, not an assertion that
changed runner bytes are identical.
