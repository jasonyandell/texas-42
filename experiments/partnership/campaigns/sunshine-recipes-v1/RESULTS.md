# Scheme recipes with deployed continuation

2026-09-13. The existing gym now accepts named continuation players. Scheme
matching, action valuation and exercise selection have separate responsibilities
and cache identities. Changing a Scheme question no longer requires replaying
already measured positions whose valuation inputs are unchanged.

The [protocol](PROTOCOL.md), [recipe](../../../../walt/gym/specs/partnership-count-l1.json)
and [usage guide](../../../../walt/gym/SPECIFICATIONS.md) define the experiment.
This is a completed instrument change and a development-corpus measurement.
**No player or live checker was changed.**

## What the L1 recipe found

The same recorded-game corpus supplied 1,929 eligible positions with two or
three tiles left in the acting player's hand. Of those, 245 exceeded the
400-world support cap, 1,478 did not match the query, and 206 matched within
the cap. All legal root actions were followed by ordinary deployed L1 at every
seat over every compatible root world: 9,188 worlds, 26,340 complete
trajectories, and 166,147 unique frozen own/public decisions.

There were 143 declaring-side matched positions and 63 defending-side ones.
Among the declaring positions, 67 tied across all legal choices. The selected
declaring-side family therefore contains 76 exercises:

| Relationship under deployed L1 continuation | Exercises | Default L1 chooses a best action |
|---|---:|---:|
| Best count offer strictly beats every non-offer | 27 | 23 |
| Best non-offer strictly beats every count offer | 46 | 36 |
| Best offer/non-offer tie, but another action is weaker | 3 | 3 |
| Total | 76 | 62 |

The ten misses in the withholding group are **six inappropriate count offers
and four choices of a weaker non-offer**. Thus the narrow offer/withhold skill
has four missed helpful offers and six harmful offers in this panel; the other
four errors concern which non-offer to choose. All equally good actions tie.
The [baseline responses and grades](baseline.json) preserve this distinction.

The baseline averaged 30.7 ms per root decision on these selected late-game
positions. This is not an ordinary-game latency estimate. Its mean root regret
was 1.258 percentage points over these 76 equally weighted selected positions,
under the specified continuation and prior. It is not a game win-rate estimate.

## The change of continuation matters

The original teacher specification reproduced **all 30 original coordinates
and complete answer keys exactly**. It still uses a lawful optimal focal
continuation, its fixed L1 teammate, L0 opponents and its declared seed schedule.
The deployed recipe uses ordinary L1 afterward at every seat and the live
wrapper's request/seed behavior. These assumptions changed together; this run
does not isolate which change causes each reversal.

Across the 206 common evaluated coordinates:

- 87 changed at least one action value.
- 35 changed the set of best actions.
- 25 changed the query's helpful/harmful/tied/no-contrast relationship.

Comparing the **same required-offer selection rule**, 24 of the old 30 remain,
six leave, and three new positions enter: 27 required-offer exercises under L1.
The [comparison report](teacher-vs-l1-required.md) retains each change and paired
terminal witnesses. This is a change in what question the answer key answers,
not evidence that either continuation model is universally right.

All **45 roots from the previous targeted replay** reproduce their deployed
action values and complete best-action sets exactly in this generic recipe.
That includes the two useful original missed offers and the four that reverse.
The generic connection agrees with the earlier specialized instrument.

## Query iteration, interruption and verification

The full generation took 283.4 seconds across three capped invocations,
including a deliberate interruption, resume and publication. SIGINT stopped
scheduling; all 9,774 already durable decision/trajectory/value records survived
byte-for-byte. Time allowance exhaustion also resumed cleanly. No trajectory
failed in the completed full run.

| Variant on the saved valuations | Exercises | Seconds | New deployed value keys |
|---|---:|---:|---:|
| Required count offers | 27 | 12.02 | 0 |
| Avoided count offers | 46 | 10.79 | 0 |
| Required offers, best-class gap at least 1/20 | 23 | 11.10 | 0 |
| Scheme changed to five-count offers only | 25 | 11.45 | 0 |

All 192,693 value/trajectory/decision records stayed byte-for-byte unchanged
through those variants. The five-count query matched 96 of the already valued
positions. Its 25 exercises are a subset of the original 76. On those 96 common
valuations it changed nine target-class relationships but **no action values or
best-action sets**: exactly the intended separation between question and value.
The [query comparison](count-vs-five.md) and [larger-gap comparison](required-vs-strong.md)
record the differences. Timings include independent audit and publication.

The independent verifier replayed all 206 complete value keys, checking support,
legal play, own/public information locality, player roles, terminal points and
make counts. A separate three-world native sample reproduced the corresponding
census traces exactly. Every root action made in two of the three sampled
worlds, so that small sample selected zero strict exercises. This is a
sampling/labeling check, not a strength result. Unit tests also check that
all-success or all-failure samples make no guaranteed-outcome claim.

All 83 partnership Python tests pass, including resume, malformed-contract
refusal, role-specific policies, corrupt-certificate refusal, query-independent
cache identity, sample coverage and renamed-recipe publication. The native
players were unchanged. Full repository CI was not run for this Python gym
change.

## What this gives the next sunshine cycle

The question can now say **which future players it means**. We can refine the
Scheme expression, inspect what enters and leaves the family, and retain both
beneficial interventions and counterexamples without paying to replay unchanged
continuations. The corpus includes the nonmatches, ties and exclusions, rather
than only success stories.

The next player task is an affordable check that predicts these deployed
consequences. Keep the baseline available, treat an exhausted check as unresolved,
and measure both missed opportunities and harmful interventions. A rare skill
can still matter to a human partner; ordinary-game frequency is not its only
acceptance criterion. New source-deal groups should challenge any chosen repair
before promotion, and the arena should check broader side effects and cost.

The live optional count reviewer still uses the older teacher-style valuation.
This work provides aligned exercises for repairing it; it does not silently
replace or promote it.

## Durable evidence and reproduction

- [Validation, resolved recipes and root agreement](validation.json).
- [Capped run receipts](run-receipts.json), [sample audit](sample-validation.json),
  and [per-coordinate values and full-key hashes](value-keys.json).
- [Three portable audited examples](examples/catalog.json) and
  [paired terminal witnesses](witnesses.json), readable with the ordinary gym's
  `show`, `verify` and `run` commands using this example directory as `--gallery`.
- Full comparison rows are stored beside the Markdown reports as `.json.gz`.
- [Iteration validation program](validate.py) and [publication/audit program](publish.py).

Raw frozen decisions and trajectories remain outside git at
`/Users/jason/data/texas-42/sunshine-recipes-v1`. Their full compressed hash index
is identified by path and SHA-256 in `validation.json`. The checked-in recipe
and original source corpus regenerate the questions without listing case IDs.

The main measured implementation is commit `7abf2aee`, whose source hashes were
checked against the receipts. Later publication fixes label sample coverage,
give renamed recipes distinct collection identities, and publish query
snapshots atomically. They passed all 39 focused gym tests. The native
continuation and action values are unchanged. As with other implementation
changes, the conservative source-hash cache identity
creates a new namespace on regeneration. Previously published collections remain
readable, comparable and usable for player exams.

These are uniform-mechanical-prior values under frozen continuation players.
They do not include pre-root behavioral belief weighting, establish general
Texas 42 optimality, or turn multiple completions of one visible position into
independent games.
