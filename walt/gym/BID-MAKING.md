# Find the bid-making play

2026-09-07. Exploratory, exact finite-domain evidence under the gym's declared
belief and continuation field. [Mathematical contract](README.md#what-an-answer-means).

The [bid-making collection](collections/bid-making-v1/catalog.json) contains
**433 distinct declaring-side decisions where legal plays have different make
probabilities**. No partnership, count, suit, or other tactical motif is required.
The same gym can discover, audit, display, and grade these exercises.

## Express the domain; select by outcome

The entire structural query is [all-legal.scheme](queries/all-legal.scheme):

```scheme
(fix
  (roles (domino action))
  (out action)
  (case (own-legal action)))
```

The exact evaluator supplies every action's value. For each coordinate, with
`N` compatible worlds and exact success mass `m(a)`, define:

- `Q(a) = m(a)/N`.
- `A* = {a : Q(a) = max Q}`: **every** tied best play.
- `regret(a) = max Q - Q(a)`: cost of that root choice in make probability.
- `spread = max Q - min Q`: largest available decision swing.
- `nearest_mistake = min positive regret`: cost of the least harmful mistake;
  absent when all plays tie.

`select --criterion outcome` keeps a coordinate exactly when its spread is
positive. `--side declaring` makes the objective making 30. No minimum effect
size, shape, or point score selects the published cases. All makes tie; all
sets tie. An exercise's displayed comparison is a best/worst pair, but its key
and pupil grading retain **all** actions and the **whole** optimal set.

This selector is separate from structural query contrast. With `all-legal`,
every action matches: there is no matched/unmatched pair for the older
`advantage`/`disadvantage` classifier. The inventory's `outcomes` section reports
the new criterion; its earlier query-contrast counts still mean what they did.

## First complete sweep

Source: the existing 400-game default-player battery on 100 deals. The generic
stream produced 1,929 distinct unsettled trick-5/6 coordinates with multiple
legal plays. All contracts were 30. Uniform full compatible belief, 400-world
cap, L1-40/8 teammate, L0-8 opponents, and lawful optimal focal continuation
were unchanged from the original gym.

| Measured property | Declaring: make 30 | Defending: set 30 |
|---|---:|---:|
| Eligible coordinates | 1,024 | 905 |
| Above the exact-world cap, skipped | 187 | 58 |
| Exactly graded | 837 | 847 |
| All legal plays tie | 404 | 387 |
| Strict outcome difference | **433** | 460 |
| Exactly one best play | **367** | 420 |
| Some play succeeds in every world and another in none | **26** | 8 |

The ten-worker discovery completed in **50.616 seconds**; publication and
another independent audit of all 433 selected exercises took **16.414 seconds**.
All 1,929 items were saved, with no failed measurements or query nonmatches.
Cap exclusions are unmeasured, not failures or evidence of a tie.

The 433 published declaring exercises break down as:

| Legal plays | Best plays | Coordinates |
|---:|---:|---:|
| 2 | 1 | 191 |
| 3 | 1 | 176 |
| 3 | 2 | 66 |

There are 265 trick-5 and 168 trick-6 positions. In 142 positions, even the least
harmful nonoptimal choice loses at least 1/4 make probability. **311 positions
are outside the earlier 170-coordinate partnership gallery.** These are useful
diagnostic strata, not additional selection rules.

The portable [discovery record](collections/bid-making-v1/discovery.json) retains
the complete candidate manifest, source and executable hashes, query source,
all measured action profiles, controls, exclusions, and the raw-run location.
The published directory holds all 433 full keys and replays. Raw defending and
tied keys remain in `/Users/jason/data/texas-42/bid-making-v1`.

## Two concrete exercises

**[bid-making-17](collections/bid-making-v1/bid-making-17.json)** has 60 legal
hidden completions. Twos are trump; our remaining tiles are 2–0, 5–0, and 5–3.
We act last on trick 5, with our team at 8 and defenders already at 11.

| Play | Make probability |
|---|---:|
| **2–0** | **60/60** |
| 5–0 | 0/60 |
| 5–3 | 0/60 |

The current trick contains 4–4, 6–1, and 4–1. Trumping with 2–0 takes its six
points and preserves the lead for our remaining fives. Every other five has
already been played. Giving away this trick lets defenders cross the set
threshold immediately. The same-world comparison gains a make in all 60
worlds, with zero losses. This exercise needs no partnership descriptor.

**[bid-making-139](collections/bid-making-v1/bid-making-139.json)** illustrates
why a single preferred tile must not become the label: both 5–0 and 6–0 make
in all 200 worlds; 5–5 makes in none. Either good play earns full credit.

## Use and reproduce

Inspect one exercise or compare all recorded action values:

```sh
python3 experiments/partnership/gym.py show bid-making-17 --gallery walt/gym/collections/bid-making-v1
python3 experiments/partnership/gym.py report --gallery walt/gym/collections/bid-making-v1
```

Discover from more recorded games, then publish by outcome alone:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-bid-discovery -- python3 experiments/partnership/gym.py discover --query walt/gym/queries/all-legal.scheme --source experiments/partnership/campaigns/default-partner-battery --output /tmp/my-bid-discovery --workers 10 --seconds 240 --limit 5000
python3 experiments/partnership/gym.py inventory --source /tmp/my-bid-discovery
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-bid-publish -- python3 experiments/partnership/gym.py select --source /tmp/my-bid-discovery --output /tmp/my-bid-gallery --criterion outcome --side declaring --all
```

Use `--side defending` to publish the set-making counterpart. Discovery keeps
both sides, so this requires no new solving. Resume discovery with the same
command/output and a fresh watchdog log directory. Completed coordinates are
atomic and reusable; source/configuration changes refuse silent resume.

The existing player runner accepts this collection directly:

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-bid-pupils -- python3 experiments/partnership/gym.py run --gallery walt/gym/collections/bid-making-v1 --output /tmp/my-bid-pupils --workers 10 --seconds 240 --players l1-default l2-partner-default l2-partner-voids
python3 experiments/partnership/gym.py report --gallery walt/gym/collections/bid-making-v1 --results /tmp/my-bid-pupils
```

This pupil comparison has **not** been run for the new collection. Its natural
scores are optimal-choice rate and mean root-action regret, with paired
comparisons on identical coordinates. Hidden completions are not independent
games; multiple coordinates can come from one deal. The collection is selected
for consequential decisions, not held out or representative of general play.

## What is established

These keys establish exact best root actions **under the specified belief and
future field**. Each root action is followed by its best lawful focal policy;
the pupil is graded on its root choice. Full replays and independent support,
legality, scoring, and information-consistency checks audit attained values.
Optimality comes from the existing exact recurrence, not a second optimizer
or a new proof-assistant theorem.

One decisive root action does not establish that all later choices are
irrelevant. Finding the shortest sufficient sequence of future decisions is
a further question. These exercises supply concrete positions, lawful
continuations, and counterfactual replays for studying it.

Seventeen focused Python tests pass, including all-legal discovery/publication,
resume without recomputation, side selection, optimal ties, metadata tampering,
and an independently replayed native reproduction of the 60-world certainty
witness. Every selected fixture was re-audited at publication. No Rust engine
or player was changed; full repository CI remains waived for the session.
