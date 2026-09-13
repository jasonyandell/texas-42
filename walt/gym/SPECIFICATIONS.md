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

**Continuation-selectable recipes (2026-09-13):**
[specs/partnership-count-l1.json](specs/partnership-count-l1.json) uses the same
Scheme expression with deployed L1 continuing at every seat. It retains helpful
and harmful cases. The original teacher contract and frozen references remain
available; they answer a different continuation question.

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
result. Matching has `status.json` and item files; deployed evaluation also has
`trajectories/status.json`, complete trajectory files, and an exact-input
decision cache. SIGINT to the generator stops scheduling and drains active
jobs. A hard stop can lose active trajectories, while completed trajectories
and decisions survive. There can be up to ten active trajectories at once.
The watchdog itself enforces its ceiling by killing the workload process group.

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
| Every optimal action must offer count | `--set selection.criterion=query-required` |
| Every count offer is worse than the best alternative | `--set selection.criterion=query-avoided` |
| Best offer versus best non-offer differs by at least 1/20 | `--set selection.min_contrast=1/20` (new L1 recipe) |
| Two dominoes in our own hand | `--set 'domain.own_remaining=[2,2]'` (new L1 recipe) |
| L2 Partner as our partner | `--set evaluation.players.partner=l2-partner-default` (new L1 recipe) |
| Assess 32 shared hidden-hand worlds | `--set evaluation.sample_worlds=32` (new L1 recipe) |

Probability thresholds are inclusive exact rationals. Strict outcome difference
is always required; a zero threshold does not admit all-tied positions.
`max_optimal` counts all best plays, and never chooses a tiebreak winner.
`min_spread` and `min_mistake` measure different questions: the worst possible
mistake may be large even when another mistake costs very little.

Filters change the generated collection identity while reusing the same
evaluated coordinates. **Query matching and action values have separate
caches.** Editing Scheme, its presence threshold, the source corpus or domain
reuses applicable values for identical own/public requests. Only newly matched
coordinates need new values. A changed continuation, sampling recipe, cap or
implementation gets its own evaluation identity. Named players expand to their
full frozen configurations; binaries and wrapper sources are pinned too.
Worker count and wall allowance are scheduling settings, not value identities.
The first realized decision at an exact input is frozen, including a fallback;
this is a replayable procedure, not a guarantee of identical timing on rerun.

Every generated collection records its resolved arguments and source identity.
All prior collections remain addressable in the output directory; `latest.json`
points to the latest completed invocation, or reports that the current one is
incomplete. Empty filtered collections are valid complete results. The receipt
reports new and reused value keys. Discovery retains ties, nonmatches and world
cap exclusions even when they are absent from the selected exercises.

Compare two completed collection paths (save the first `latest.json`'s gallery
path before making a variant):

```sh
python3 experiments/partnership/gym.py compare /tmp/my-gym/collections/FIRST /tmp/my-gym/collections/SECOND --output /tmp/query-change.json
```

The command writes JSON and Markdown: added/removed coordinates, changed action
values and best-action sets, helpful/harmful/tied query relations, and paired
terminal witnesses where available. Stable coordinate IDs identify positions;
display names such as `advantage-01` may change when a collection is filtered.

## The specification's contract

`source.paths` are relative to the repository root, or absolute paths for local
external corpora. The source fingerprint covers result and checkpoint bytes,
with paths relative to each source, so relocating a checkout preserves it.
The coordinate generator still reads completed game records. It does not yet
synthesize arbitrary reachable game histories from constraints.

`domain` chooses either `tricks: [5,6]` or `own_remaining: [2,3]`, a
coordinate-count limit, and the seed placed in the pupil request. The range
is inclusive. The original examiner's field retains its fixed
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

`evaluation.contract = deployed-gym-v1` instead names deployed continuation
players separately for `focal`, `partner`, and `opponents`. Force each legal
root action, then let those ordinary players finish. Each decision receives
only its own original hand and public history, with the recipe's seed. No
optimal focal continuation is substituted. The full-support value is therefore
`P(team succeeds | forced action, named frozen continuation)`.

`sample_worlds: null` evaluates every mechanically compatible root world under
the cap. A positive integer draws that many worlds without replacement, using
the pinned `sample_seed` and coordinate; all root actions share exactly that
subset. Support must still be enumerated under `max_worlds` first: this is a
bounded late-game instrument, not an opening-position support sampler.
If the subset covers the support the key says `census`; otherwise it says
`sample-without-replacement`. Sampled values cannot establish a guaranteed
make or set, and sampled recipes refuse `selection.certain=true`.

Keys retain full terminal traces and a table of frozen own/public decisions.
Independent replay checks support, sample membership, legal play, information
locality, role configurations, terminal points, paired outcomes and all action
masses. It audits recorded decisions; it does not independently prove that a
native algorithm would always produce them under every timing realization.

`selection.criterion` supports `outcome` (any strict success-probability
difference), `query` (best action versus the opposite side of the query
target/non-target division), or `query-required` (the best matched action
strictly beats the best unmatched action, so every optimum must match), or
`query-avoided` (the best matched action is strictly worse than the best
unmatched one). Required/avoided criteria refuse empty and full target sets, which lack a
matched/unmatched comparison. Side and outcome filters apply afterward.
Scheme matching, continuation evaluation, and outcome selection remain
separate stages. `min_contrast` measures the absolute best-matched versus
best-unmatched gap; it differs from a comparison against a weaker alternative.
The broad `query` criterion can include a tied best offer/non-offer when either
beats another action. Read its explicit query contrast before calling an offer
required. `query-required` is the unambiguous positive-skill collection.

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
claims. Census values are exact under a declared belief and frozen future
players; sampled values are estimates. Both are audited by independent replay,
rather than field-independent optimality or a
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
