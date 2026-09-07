# Scheme policies: persistence helps; donor composition has not paid for itself

2026-09-07. Exploratory engineering and finite-domain experiments. Bid 30;
make/set only. [Machine-readable measurements](measurements.json) retain the
configuration, binary/source identities, per-root rows and exact gym fractions.

## Delivered instrument

Scheme now has finite typed forward transitions, explicit belief pushforward,
rigid-role persistence/extinction/birth, and finite backward preimages. Its
policy layer supplies serializable own/public-information guards, legal action
selection, rigid bindings, persistent controller modes and explicit fallbacks.
Generated policies are saved, parsed, compiled and independently replayed.

The new constructor uses the existing rules/kernel and fixed-field interfaces.
It compares three methods on identical growing samples: a fresh full search,
the same search retaining completed subproblems, and composition using the
union of every successful singleton-donor action at each information state.
Composition re-solves the joint sample with shared information-state choices;
it does not select a separate policy by hidden world. See the
[algorithm and preservation argument](../../../../walt/scheme/COMPOSITION.md).

The donor traversal retains successful alternatives rather than only the first
winning witness. With complete donors it preserves the full training optimum.
Accordingly, this comparison tests computational savings and executable-policy
parity; it does not presume better playing strength than an exact fresh solve
of the same sample.

## Opening experiments

Each seed starts from an independently shuffled full deal. The focal player
holds seven dominoes and is about to lead the first trick, with sixes declared
and a bid of 30. Each root gets its own training stream and an independent test
stream. Sampling is with replacement: independent draws can coincide in
physical world identity. No training/test filtering uses outcomes.

| Panel | Independent roots | Training schedule | Test worlds per root | Whole campaign wall |
| --- | ---: | --- | ---: | ---: |
| Cheap fixed `hash-legal-v1` field | 32 | 1, 2, 4, 8, 16, 32, 64, 128, 200 | 256 | 17.189 s |
| Existing native L0-8 field | 12 | 1, 2, 4, 8, 16, 32 | 64 | 64.590 s |

Ten workers were used. Whole-campaign wall includes serialization and replay
audits; summed arm times below measure search work under parallel load and are
not the same quantity. Field types are separate experiments, not a head-to-head
strength comparison.

Across every completed three-arm comparison, serialized policy identities,
training outcome vectors and test outcome vectors agree exactly. This is 287
opening-hash comparisons and 72 L0 comparisons. One opening root hit the
2,000,000-node ceiling at 200 samples in fresh and compose; persistence completed
that root. Both refused arms explicitly evaluated their last completed
128-sample policy. They are excluded from the balanced timing comparison.

| Complete paired schedules | Fresh search time | Persistent search time | Saving | Compose search time |
| --- | ---: | ---: | ---: | ---: |
| Hash field, 31 roots | 20.531 s | 9.664 s | **52.9%** | 21.831 s; 6.3% more than fresh |
| L0-8, 12 roots | 114.369 s | 61.298 s | **46.4%** | 129.513 s; 13.2% more than fresh |

Persistence saved 56.0% and 49.4% of nodes respectively. At the final stage,
mean search time was 302 ms fresh versus 119 ms persistent at 200 samples in
the cheap field, and 4.744 s versus 2.557 s at 32 samples in L0-8. These are
construction times at these roots, not timings for the existing L1/L2 wrapper.

Composition slightly reduced final-stage work in parts of the opening panel,
but paying for all singleton donors erased that benefit over the full growing
schedule. It currently produces the same program at greater overall cost.

## Generalization is the remaining issue

All 32 persistent opening runs completed at 200 samples: training makes were
4,979/6,400 (77.8%), versus 2,100/8,192 (25.6%) on independent test draws. The
L0 panel finished at 335/384 (87.2%) training versus 182/768 (23.7%) test.
These large gaps expose sample fitting; they are not calibrated estimates of
the optimal lawful make probability.

Increasing from one sample to the declared final sample count increased test
makes from 1,777 to 2,100 in the hash panel and from 157 to 182 in L0. These
are descriptive paired observations over 32 and 12 root units, not thousands
of independent game seeds or a settled strength claim. Intermediate stages
were not selected as winners.

The emitted programs are exact information-state tables with a lowest-legal
fallback on unseen histories. A typical 200-sample opening program has roughly
880 stored decisions and 0.34 MB of text. The policy language also supports
relational guards, but this constructor does **not** infer general Scheme
rules from its tables. It does not yet train one generalized pre-deal player
across different starting hands. The random-deal panel tests the construction
method across starting hands, independently at each root.

## Existing partnership gym

All 30 composed partnership/bid-making exercises completed, producing 360
rows in 2.118 seconds. The field is exactly the gym's fixed L1 teammate/L0
opponent field. Every serialized policy was evaluated over its entire root
fiber, and no policy exceeded the exact value for its chosen first action.
All three methods produced identical programs at every stage.

| Training worlds | Optimal first actions | Mean first-action regret | Mean complete-policy gap to optimum |
| ---: | ---: | ---: | ---: |
| 1 | 16/30 | 9.763 percentage points | 12.322 percentage points |
| 4 | 22/30 | 2.015 pp | 3.551 pp |
| 16 | 25/30 | 0.616 pp | 1.299 pp |
| 64 | **26/30** | **0.436 pp** | **1.008 pp** |

Each row averages the 30 coordinates equally. First-action regret gives the
chosen move the best lawful continuation; the complete-policy gap evaluates
the actual saved continuation, including its fallback. They are different
scores. The shared gym field cache makes this a quality panel; its arm timing
is not used for speed claims. These 30 exercises come from 21 source deal
seeds and remain a narrow late-game diagnostic.

## Reproduction and validation

The [instrument guide](../../POLICY-SYNTHESIS.md) supplies commands and resume
semantics. Raw artifacts and policy programs are retained at:

- `/Users/jason/data/texas-42/policy-synthesis-opening-v1`
- `/Users/jason/data/texas-42/policy-synthesis-l0-v1`
- `/Users/jason/data/texas-42/policy-synthesis-gym-v1`

Configuration/source/binary identities are checked on resume. Completed seeds
are atomic; interrupted workers may retry their in-flight seed. Node-budget
refusals retain the last complete policy and are explicitly labeled.
The raw runner's `persistent_value_mismatches` count includes the one comparison
against a refused incumbent; complete-only analysis above finds zero failures.
The shipped runner corrects that reporting issue and a macOS exit/interrupt
cleanup race discovered during verification. The shipped CLI also handles the
full unsigned-64-bit seed range without overflowing its arm-order calculation.
The measured runner and CLI sources are preserved in `reference/`; the measured
native executable is retained under
`/Users/jason/data/texas-42/policy-synthesis-producers/` with its SHA-256 in its
filename. These final boundary/cleanup fixes do not change the search algorithm.

Focused tests cover typed predecessor legality, belief odds, role identity,
parser/controller errors, hidden-information exclusion, full-history keys,
sample multiplicity, fresh/persistent parity, donor alternatives, cache scope,
independent policy repricing, and runner corruption/timeouts/resume. Strict
native lint checks passed. Full legacy Rust CI remains waived for this session.

The [verification receipt](verification.json) records a real interruption after
one of six seeds completed: the runner exited cleanly with incomplete status,
left no native workers, and resumed to six completed seeds with the original
completed result byte-for-byte unchanged. An independent Python rules replay
also passed all 222 retained full-game traces across the three panels.
