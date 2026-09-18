# Whole-game witnesses, reversible sample estimates, no accepted new rule

Exploratory, 2026-09-18. **The sampling rig now reaches from seven dominoes
remaining through two. It preserves both the action change and the changed
hidden holdings, derives executable descriptions from those contrasts, and
tests them on fresh hands.** None of the first three selected descriptions
passed the registered fresh gate. The broader evidence and reproducible
instrument are the result; a stronger player or sample-efficiency gain is not.

[Protocol](../WHOLE-GAME-CONTRASTS-V1.md) · [runner](../whole_game.py) ·
[miner](../whole_game_mine.py) · [publisher](../whole_game_publish.py) ·
[frozen fit](fit.json) · [fresh test](result.json) · [summary](summary.json) ·
[72 witnessed contrasts](witnesses.json) · [sample history](refinement.json).

## What we sampled

Each panel has 120 declaring-team positions: 20 at each own-hand size 7..2,
selected by fixed hash without looking at alternative-action outcomes. Both
bidder and partner turns are eligible. The bid is 30 and still unsettled,
with at least two legal actions. Development uses source deals 420600..420609;
fresh uses 420645..420654. Each comes from 360 complete games across four
bidder hands and nine declarations. There were 2,872 / 2,788 eligible roots.

Every root draws eight uniform, void-respecting mechanically compatible worlds
with replacement. The existing integer capacity-DP sampler reaches opening
supports as large as **399,072,960 worlds without enumerating them**. Repeated
draws remain legitimate observations. Uniform mechanical support omits policy
likelihoods from earlier actions; it is the declared prior for this study.

The baseline is the **original saved played move and receipt**, including its
160-world opening evaluation where applicable. Every legal root action is
forced in the same sampled world, followed by ordinary shared Walt at all four
seats: 40 worlds, partner review, 14-second budget, original public policy seed.
Each actor gets only its own original hand and public history. Sample randomness
is independent of policy randomness. These are comparisons under specified
Walt continuations, not perfect-information values or optimality claims.

## Early positions provide plenty of contrasts

Fresh panel, eight worlds per root:

| Own dominoes remaining | Roots | Roots with a sampled make/set action flip | Roots with an apparently better alternative mean |
|---|---:|---:|---:|
| 7 | 20 | 20 | 14 |
| 6 | 20 | 19 | 9 |
| 5 | 20 | 15 | 4 |
| 4 | 20 | 12 | 4 |
| 3 | 20 | 12 | 3 |
| 2 | 20 | 9 | 1 |

The last column maximizes over noisy eight-world estimates and is therefore
optimistic. It is a source of study candidates, **not a measured mistake rate**.
Early roots also offer more actions. Still, the full-game sampling instrument
does find concrete action-sensitive examples outside the endgame.

## What changing the sample actually changed

We deepened 24 development roots, four per size, from 8 to 24 worlds. Selection
favored apparent alternative gains and then discordance, as registered. This
is diagnostic refinement, not an independent confirmation set.

- All **960 original world receipts retained their exact hashes**.
- Eighteen selected roots initially favored a particular alternative over the
  original move. Keeping that alternative fixed, **13 remained positive, two
  became tied, and three became negative** at 24 worlds.
- The empirical preferred move changed at **8 of the 24** roots.
- Every prefix, every new helpful/harmful/tied world and its complete action
  continuations remain recoverable. The [history](refinement.json) publishes
  all 24 curves, action means and representative new-world traces.

A concrete opening example: source deal 420603, seat 2 bidding in **twos**,
root `9a6f580b170579bdb6391bfabff22582ba825969a47954c5e360aa3ce140aa1e`.
Walt originally leads **2-2**; the alternative is **3-2**.

| Worlds | Alternative helps | Alternative hurts | Ties | Net sampled make difference |
|---|---:|---:|---:|---:|
| First 8 | 3 | 0 | 5 | +3/8 = +37.5 pp |
| Next 16 only | 0 | 4 | 12 | -4/16 = -25 pp |
| All 24 | 3 | 4 | 17 | -1/24 = -4.17 pp |

The four harmful additions have zero-based sample indices **11, 18, 22, 23**.
At index 11 the original lead makes 34; the alternative sets. Both trajectories
are in the published history and complete original responses are in the durable
receipt. This reversal does not establish which lead is truly best; it gives
us specific counterexamples to the initial estimate rather than losing that
estimate when more samples arrive.

## What descriptions traveled to new hands?

At a fixed root/action pair define `D = made(alternative) - made(original)`.
The miner selected 72 witnessed pairs of worlds with different D, twelve per
hand size, and examined tiles whose owner changed between those worlds.
It lifted them into **220 shared role/property/action-relation Schemes**.
There are no physical tile IDs or hand-size conditions in a learned expression.
Each nominated description must distinguish its own originating witness pair.

Ranking uses D centered within its exact root/action pair, so a generally bad
alternative or hard hand cannot alone earn a description. Of 212 eligible
descriptions, three were frozen (commit `86792586`) before fresh alternative
labels. All raw candidates, sources, origins and signs remain in [fit.json](fit.json).

The effect below is the mean **centered action difference among matching
worlds**. It is not a direct recommendation or player win-rate improvement.

| Witness-derived description | Development effect | Fresh effect | One-sided permutation p | Registered gate |
|---|---:|---:|---:|---|
| Partner holds live top trump that beats the alternative | -4.39 pp | -3.92 pp | .0685 | Inconclusive |
| Next opponent holds live top trump that beats the alternative | +3.95 pp | -2.80 pp | .9145 | Reversed |
| Previous opponent holds a 10-count tile beating neither action | +4.64 pp | -1.81 pp | .7315 | Reversed |

All three match all ten fresh deals and all six hand sizes. The gate required
the frozen direction, at least 5 pp, coverage, and p <= .05/3. Permutations
shuffle sample indices jointly across all alternatives of each root, preserving
their dependence. Fresh per-size effects are retained and are not uniform:
for example, the partner-top-trump association reverses at size five.

"Beats" here is a mechanical rank comparison in the current led context, or
the original move's lead context at a lead. It does not mean the hidden tile
will be legally playable, or that its holder will capture a later trick.
These simple descriptions may omit the continuation relationship that matters.
The cross-world ownership differences nominate possible explanations; they do
not isolate a single causal tile. Hidden predicates are offline world/belief
queries and never actual hidden-hand inputs to the live player.

## Throughput, verification and reuse

Eighteen single-threaded workers on this Mac:

| Work | Completed | Wall seconds |
|---|---:|---:|
| New fresh source games | 360 | 51.1 |
| Development first eight | 960 all-action worlds | 57.9 |
| Development additions | 384 all-action worlds | 28.5 |
| Fresh first eight | 960 all-action worlds | 53.4 |

That is **2,304 sampled worlds, 9,200 complete forced-action branches and
171,792 ordinary continuation moves**, plus the fresh source games. Every
sample, public prefix, legal move, actor input and terminal score passed the
independent replay audit; no worker errors, deadline overruns or fallbacks.
Native Scheme evaluation agrees with Python on **46,452 memberships**.
Four new sampling tests, three mining tests, seven existing decision tests and
five Rust sampler tests passed. These audits check this implementation and
recorded evidence, not a theorem of strategic strength.

Each queue item is one world with all its legal actions. Completed items commit
atomically; interrupted items can be retried without changing their seed.
Minute-bounded runs stop cleanly and resume pending items. Full source games,
all action traces and original responses, binary/source identities and all
sample receipts live at:

`~/data/texas-42/kiln-played-v1/whole-game-contrasts-v1/`

The rig now supplies the wider discovery domain requested. A subsequent grammar
can study relationships along these saved continuations, with new confirmation
hands after fitting. This run does not yet demonstrate learned threats,
improved sampling or stronger play. The production campaign stays stopped and
the phone player is unchanged.
