# Let Scheme drive discovery

2026-09-07. The miner now takes a Scheme/Fix expression as its matching rule.
The expanded gallery contains **170 distinct late-game coordinates with strict
outcome differences**, expressed through three query files. The original six
fixtures remain unchanged as the starter gallery.

**Next application:** [outcome-only discovery](BID-MAKING.md) removes the
partnership descriptor entirely. An all-legal query and exact-value selector
publish 433 declaring-side bid-making exercises from the same source corpus.

## What changed

The first miner performed the count-offer test in Python, then checked the
same property through a Scheme query calling one custom `offer-count-to-partner`
predicate. It was a working exercise finder, but the pattern was mostly hidden
inside procedural code. It did not yet implement “write the relation in Scheme
and let the generic machinery search.”

`gym.py discover` now streams legal late-game coordinates through the supplied
query. That stream filters only by trick number, whether multiple legal plays
exist, and whether the contract is unsettled. **No partnership pattern is coded
into the coordinate stream.** Query results determine which actions and which
coordinates proceed to exact grading. The old miner and custom predicate remain
available to reproduce the starter experiment.

Three small, general public relations were added to the standard registry:

- `own-legal(action)` — action is legal for the viewer, who must be next to act.
- `trick-play(seat, tile)` — this seat played this tile in the current partial trick.
- `leads-context(tile, context)` — the context this tile would lead under the declaration.

These combine with the existing roles, `partner`, `current-winner`, `beats`,
`count`, `holds`, and `boss`. All three families below are ordinary Scheme
expressions; none needs its own matcher implementation.

## The three expressions

| Query | What it expresses | Information used |
|---|---|---|
| [offer-count.scheme](queries/offer-count.scheme) | A legal count play leaves our currently winning partner ahead. | Public state and own hand |
| [overtake-partner.scheme](queries/overtake-partner.scheme) | A legal play takes the current lead from partner. | Public state and own hand |
| [lead-to-partner-boss.scheme](queries/lead-to-partner-boss.scheme) | At our lead, partner holds the live boss of the context our candidate would lead. | A relation evaluated across the entire legal belief |

The first query says, in relational form:

```scheme
(fix
  (roles (chair me) (chair mate) (domino action)
         (domino incumbent) (context led))
  (out action)
  (case (viewer me) (partner me mate) (current-winner mate)
        (trick-play mate incumbent) (led-context led)
        (own-legal action) (count action 5)
        (beats incumbent action led))
  (case (viewer me) (partner me mate) (current-winner mate)
        (trick-play mate incumbent) (led-context led)
        (own-legal action) (count action 10)
        (beats incumbent action led)))
```

Five- and ten-count clauses are the two cases of the Fix. `action` is returned;
the other roles are existential witnesses. Neither repeated witnesses nor
overlapping cases multiply answers or probability mass.

The query language expresses a mechanical opportunity. Whether taking it
improves make/set probability remains the exact evaluator's separate job.
Leading toward a partner boss, for example, does not guarantee that partner
wins against a possible off-suit trump, nor that this is the right time to lead.

## Results of the first generic sweep

The source was the saved default-player battery: 1,929 distinct, eligible
trick-5/6 coordinates, deduplicated across its two campaigns. Each query saw
the same coordinates. The exact world cap was 400. All contracts were 30;
the fixed future field and root-action grading remained the starter exam's.

| Query | Matched and exactly graded | Target action helps | Target action hurts | No strict target/comparison pair |
|---|---:|---:|---:|---:|
| Offer count | 206 | 66 | 51 | 89 |
| Overtake partner | 94 | 17 | 18 | 59 |
| Lead to possible partner boss | 53 | 4 | 20 | 29 |

The 176 strict query/coordinate memberships represent **170 unique coordinates**;
six coordinates occur in more than one query family. The combined catalog
deduplicates them and retains all memberships. Duplicate coordinates had
identical underlying requests and exact action keys across families.

Each query skipped the same **245 above-cap coordinates**, whose pattern status
was not determined. Among the remaining coordinates, respectively 1,478, 1,590,
and 1,631 did not meet the query-presence criterion. There were **no failed
measurements**. Nonmatches and matched controls are retained in the discovery
summaries, not silently discarded.

The three searches ran together with 4 + 3 + 3 workers, ten total. Their capped
durations were **13.048, 10.803, and 12.634 seconds**. This is runtime for a bounded
late-game sweep over existing records, excluding compilation and later gallery
publication. It does not estimate opening-state search or all of Texas 42.

All strict keys and their full replays are checked in under
[collections/scheme-v1](collections/scheme-v1/catalog.json). The shared
[coordinate manifest](collections/scheme-v1/coordinates.json) and each family's
`discovery.json` preserve the complete scan denominators, query source, threshold,
source identities, match counts, controls, and action-value summaries. Nonselected
full raw traces remain in the local directories recorded by those summaries.

These positions come from saved games and were selected by outcome differences.
They are an expanded diagnostic gym; neither a representative frequency estimate
nor a new L1/L2 strength ranking follows from these counts. No expanded pupil
matchup was run in this discovery step.

## Belief queries without hidden-hand leakage

The first two expressions use only predicates registered with Viewer access.
Their presence is constant across the full legal belief, so matching requires
one world evaluation plus the exact total count. The third uses `holds`; it is
evaluated over **every** compatible hidden world up to the explicit cap.

`--min-presence 1/4` selects actions for which at least a quarter of the declared
uniform belief supports the relation. This is an explicit discovery threshold,
not a statistical confidence claim or an undisclosed conditioning event.
Exact grading still uses the **entire original legal belief**.

A concrete stored example, coordinate `1bdec02c98fb0deb9df4`, has 140 legal
worlds. Leads 6–0 and 6–4 each connect to a partner boss in exactly 40 worlds,
so each has presence 2/7. Both meet the quarter threshold. The answer key still
grades all 140 worlds; it does not hand the player a magically certain partner
boss. A regression test checks the fractional presence and reproduces the
unchanged full-belief action key.

The pupil continues to receive only its original hand and public history.
Query source, labels, witnesses, and actual hidden hands remain examiner data.

## Run, inspect, and extend

Read the expanded collection without running a player:

```sh
python3 experiments/partnership/gym.py report --gallery walt/gym/collections/scheme-v1
python3 experiments/partnership/gym.py show offer-count--advantage-01 --gallery walt/gym/collections/scheme-v1
```

Run a new query sweep, then publish its strict exercises:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-discovery-1 -- python3 experiments/partnership/gym.py discover --query walt/gym/queries/offer-count.scheme --source experiments/partnership/campaigns/default-partner-battery --output /tmp/my-discovery --workers 10 --seconds 240 --limit 5000
python3 experiments/partnership/gym.py inventory --source /tmp/my-discovery
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 120 --output-dir experiments/partnership/runs/my-discovery-select -- python3 experiments/partnership/gym.py select --source /tmp/my-discovery --output /tmp/my-gallery --all
```

For the belief query, substitute its file and add `--min-presence 1/4`.
`--limit` counts coordinates inspected, not successful discoveries. Multiple
source directories are supported. `--min-trick`/`--max-trick` choose within
tricks 5–6. A source must contain completed result/checkpoint records in the
existing campaign format. More recorded games expand the search domain.

Supply your own `.scheme` file to change the pattern. The gym query must return
exactly one domino action; other roles stay existential. Constrain returned
actions with `own-legal`. Bad output types, illegal returned actions, unsupported
queries, and exhausted query budgets refuse complete results. Compile checking
runs before workers start; query execution has a finite work budget.

The output directory freezes the query source as `query.scheme`. Changing the
query, threshold, source inputs, engine, or runner refuses silent resume. Repeat
the same command with a fresh watchdog output directory to resume an unchanged
experiment. `status.json`, per-item atomic saves, failure retries, and interrupt
handling are shared with the existing gym runner.

Existing combinations of registered relations need only a new expression.
A new mechanical relation can be registered with types, information access,
version, and a bounded horizon. A claim about a multi-trick continuation needs
that continuation relation defined and checked. Scheme does not invent
reachable histories, synthesize new predicates, or turn a structural match
into a proven beneficial move on its own.

## Validation

Eight focused gym Rust tests and the 19 Scheme tests pass, including expression
equivalence to the original offer predicate, seat rotations, fractional hidden
presence, output/legality refusal, and finite-budget refusal. Python tests check
declarative offer equivalence on all 60 original mining roots, the generic
coordinate stream, target-driven grading, the 170-coordinate catalog,
fractional-presence/full-belief separation, and the prior replay/resume gates.
Every published exercise was independently replay-audited and its Scheme match
recomputed during publication. Full repository CI remains waived for this session.
