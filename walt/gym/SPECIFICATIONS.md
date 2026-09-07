# A collection is a specification

2026-09-07. The maintained definition of the bid-making family is now
**[specs/bid-making.json](specs/bid-making.json)**. Its arguments generate the
exercises. The 433 checked-in fixtures are the first materialized reference
result, rather than 433 separately maintained exercise definitions.

The specification contains the recorded-game source, coordinate domain,
embedded Scheme query, evaluation contract and budgets, and exact outcome
selection. It contains **no list of the 433 position IDs or answer labels**.
A small reference section records fingerprints used only to check reproduction.

The [first composed family](PARTNERSHIP-COMPOSITION.md) combines the partnership
count-offer Fix with a required-action-class outcome condition, yielding 30
positions and a completed default L1/L2 Partner exam.

## Generate and use it

From the repository root, run one generation command under the session's
existing wall-time watchdog:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-spec-1 -- python3 experiments/partnership/gym.py generate --spec walt/gym/specs/bid-making.json --output /tmp/my-gym --workers 10 --seconds 240
```

The same command both discovers and publishes. Repeat with the same output
directory and a fresh watchdog log directory to resume. Failed coordinates
retry; completed evaluations are preserved. A partial generation cannot be
used as a complete collection. `latest.json` records progress or the completed
result; each evaluation also has the existing `status.json` and item files.

Use the output root directly; no need to copy its internal hashed path:

```sh
python3 experiments/partnership/gym.py show bid-making-17 --gallery /tmp/my-gym
python3 experiments/partnership/gym.py report --gallery /tmp/my-gym
```

The existing `verify` and player `run` commands accept that same `--gallery`.
The latter still needs the watchdog and its own resumable player-output directory.
No hidden examiner fields enter the pupil request.

## Change the arguments

Add one or more `--set path=value` arguments to `generate`. They modify the
invocation without editing the specification. Reuse the same output root.

| Desired collection | Argument |
|---|---|
| Exactly one best legal play | `--set selection.max_optimal=1` |
| Best/worst make-probability difference at least 1/4 | `--set selection.min_spread=1/4` |
| Every nonoptimal choice loses at least 1/4 | `--set selection.min_mistake=1/4` |
| A certain success versus certain failure alternative | `--set selection.certain=true` |
| Defending-side set-making decisions | `--set selection.side=defending` |
| Only trick 6 | `--set 'domain.tricks=[6,6]'` |
| A larger exact-world cap | `--set evaluation.max_worlds=1000` |
| Different recorded games | `--set 'source.paths=["path/to/campaign"]'` |

Probability thresholds are inclusive exact rationals. Strict outcome difference
is always required; a zero threshold does not admit all-tied positions.
`max_optimal` counts all best plays, and never chooses a tiebreak winner.
`min_spread` and `min_mistake` measure different questions: the worst possible
mistake may be large even when another mistake costs very little.

Filters change the generated collection identity while reusing the same
evaluated coordinates. Changing the source, domain, query, evaluator settings,
or implementation creates a separate evaluation cache. Worker count and wall
allowance affect execution scheduling, not mathematical identity.

Every generated collection records its resolved arguments and source identity.
All prior collections remain addressable in the output directory; `latest.json`
points to the latest completed invocation, or reports that the current one is
incomplete. Empty filtered collections are valid complete results.

## The specification's contract

`source.paths` are relative to the repository root, or absolute paths for local
external corpora. The source fingerprint covers result and checkpoint bytes,
with paths relative to each source, so relocating a checkout preserves it.
The coordinate generator still reads completed game records. It does not yet
synthesize arbitrary reachable game histories from constraints.

`domain` chooses a trick range within 5–6, a coordinate-count limit, and the
seed placed in the pupil request. The examiner's field retains its fixed
420600/state-derived seed schedule under the named evaluation contract;
changing the request seed does not change that teacher. Bid 30, an unsettled contract, and multiple legal
choices are part of this generator's versioned contract. The limit counts
coordinates inspected, including ties and cap exclusions, rather than matches.

`query.source` is ordinary Scheme/Fix text, embedded so the recipe is one file.
The output must be a legal domino action. `query.min_presence` is exact presence
over the whole compatible belief; it never narrows the grading belief.

`evaluation.contract = partnership-gym-v1` names the existing
[gym semantics](README.md#what-an-answer-means): uniform mechanically compatible
root belief; lawful optimal focal continuation; fixed L1 teammate with eight
inner L0 worlds and voidless inner modeling; L0-8 opponents; make/set only.
`partner_worlds` controls the teammate's outer sample count (40 by default),
while `max_worlds` is the exact root-support cap. These are distinct quantities.
The contract and every implementation/binary identity are saved in the run.
An unknown evaluator contract is refused, never silently approximated.

`selection.criterion` supports `outcome` (any strict success-probability
difference), `query` (best action versus the opposite side of the query
target/non-target division), or `query-required` (the best matched action
strictly beats the best unmatched action, so every optimum must match).
The required criterion refuses empty and full target sets, which lack a
matched/unmatched comparison. Side and outcome filters apply afterward.
Scheme matching, exact grading, and outcome selection remain separate stages.

The baseline reference pins the default arguments and source, then checks the
**same position set and complete native answer keys**, including replays.
It is checked after generation and never supplied to the matcher or solver.
Matching the number 433 alone is insufficient. Ordinary metadata such as timing
and a new runner hash is excluded from answer equivalence.

An argument override creates an explicit variant: the baseline expectation no
longer applies. To maintain a different default recipe, copy the specification,
change its arguments, and remove or deliberately replace `reference`. Editing
defaults while retaining an incompatible reference refuses the run. Expanding
the same source directory likewise requires acknowledging its new reference.

## Evidence boundaries

Specifications organize experiments; they do not strengthen the evaluator's
claims. These remain exact values under a declared belief and future field,
audited by independent replay, rather than field-independent optimality or a
formal proof of all game mechanics. Selected exercises and hidden completions
are not independent random games. All best plays tie for grading.

The original [bid-making results and witnesses](BID-MAKING.md) remain the
reference record. Generated exercise data and caches belong outside the source
tree by default; new family definitions belong under `specs/`.

## Reproduction record

The [validation receipt](specs/bid-making.validation.json) records a fresh run
from the specification, followed by two argument variants:

| Invocation | Exercises | Seconds |
|---|---:|---:|
| Default specification, fresh evaluations | 433 | 67.707 |
| `selection.max_optimal=1`, cached evaluations | 367 | 13.991 |
| `selection.certain=true`, cached evaluations | 26 | 1.921 |

The default position-set and complete answer-key fingerprints match the
original reference exactly. Both variants used the same evaluation identity,
and the saved item files were byte-for-byte unchanged afterward. Filtering
time includes independent re-auditing and publication of selected exercises.
No new player comparison was run.

Twenty-two focused Python tests passed, covering schema/type refusal, rational
thresholds, reference drift, portable source identity, failed-coordinate retry,
cache sharing, incomplete-collection refusal, empty results, and the previous
gym replay/legality/information gates. The native engine was unchanged; full
repository CI remains waived for this session.
