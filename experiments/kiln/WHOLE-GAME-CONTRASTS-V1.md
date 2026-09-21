# Whole-game sampled contrasts

Exploratory protocol, 2026-09-18, before alternative-action measurements.
The two-tile experiment is a verified instrument, not our strategy-discovery
domain. This cycle samples every meaningful hand size, seven through two.

## Evidence first

We have concrete differences already: worlds, changed holdings, forced moves
and complete resulting trajectories. The unknown is a reusable relationship
explaining those differences. Keep these witnesses; do not replace them with
an assumed tactic, an attention model or a late-game-only policy grammar.

## Source, sampling and interventions

- Development: deals 420600..420609, trial zero, four bidder hands and nine
  declarations from the completed actual-play corpus.
- Fresh: new deals 420645..420654, the same 360-game source design and pinned
  full-game producer. No fresh alternative-action labels are opened until fit.
- Eligible: declaring-team turns with an unsettled bid30 contract, at least two
  legal moves, and own remaining size 7,6,5,4,3,2. Select two coordinates per
  source deal/size by a fixed hash, without outcome or threat filtering. Keep
  coverage deficits rather than silently filling them from other stages.
- The baseline is the **original saved deployed move**, whose full request and
  response are retained. It is not chosen afresh by a cheaper model. Source
  opening moves use 160 worlds; later moves use 40 and partner review.
- For each coordinate draw eight independent uniform mechanically compatible
  worlds **with replacement**, using the existing integer capacity-DP sampler.
  Every sample index has its own seed, independent of player randomness. Never
  enumerate opening support, deduplicate repeated draws, or condition on an
  outcome. The prior respects observed voids but omits behavioral likelihoods.
- Force every legal root action in the same sampled world, then finish with
  ordinary shared deployed Walt at all four seats (40 worlds, partner review,
  14 seconds). The original public policy seed is held fixed across worlds and
  actions. No hidden examiner fields enter an actor's request.
- One complete all-action sampled world is the atomic, resumable job. Preserve
  original responses and complete traces, including ties, harms and errors.
  Only complete planned prefixes are eligible for analysis. Runs are parallel
  and minute-bounded under the existing watchdog.

## Derive descriptions from outcome contrasts

Each row is `(coordinate, alternative, world, D)`, where
`D = made(alternative) - made(original)`, in {-1,0,1}. Compare worlds at the
**same coordinate and same action pair** with different D. Select at most 12
contrast pairs per hand size by a fixed hash. For each pair, retain every tile
whose hidden holder changed and both full continuations. This is a nomination
mechanism, not a claim that one of those tiles alone caused the effect.

Lift those witnessed tiles into a shared finite relational vocabulary:
holder relative to viewer (after/partner/before), zero or one tile property
(called/not, double/not, count 0/5/10, live top trump/not), and one or both
relations describing whether it beats each root action in the current led
context (or the original action's lead context at a lead). Names of physical
tiles and the remaining-hand size are absent from the resulting Scheme.
Emit genuine Scheme expressions with `(alternative, baseline)` output roles.
The grammar and witness filter are fixed here; selection uses development only.

Rank association with D **after subtracting the mean D of that exact coordinate/
action pair**. Thus generally hard hands or obviously bad alternatives do not
by themselves earn a pattern. Require >=64 matching rows, six source deals,
three hand sizes, and variable membership in >=12 coordinate/action pairs.
Rank squared signed residual sum divided by matches; retain up to three
patterns, suppressing pairs whose membership Jaccard overlap is >=4/5.
Freeze source, sign, witnesses and the full candidate table before fresh testing.
An empty fit is a valid result. Patterns remain examiner/belief queries, not
direct hidden-information move guards or deployed policies.

## Fresh checks and progressively added games

Primary: does each frozen pattern retain its signed conditional D association
on the new deals? Report pooled and per-size coverage/effects, raw matched and
unmatched D, and a 1,999-draw permutation check. Permute sample indices jointly
across all alternatives at a coordinate; do not treat their outcomes as
independent games. A provisional replication needs the same direction, >=.05
signed residual per match, >=5 deals, >=3 sizes and p<=.05/number selected.
This tests an outcome relationship, not stronger play or sample savings.

Separately deepen up to four development coordinates per size from 8 to 24
worlds. Choose positive observed best-alternative gain first, then paired
discordance, then a fixed hash; exclude wholly outcome-equivalent roots.
Keep the first eight immutable. Report how action estimates change and retain
the exact new worlds responsible. This selected refinement is diagnostic, not
fresh confirmation. It does not alter the frozen Scheme discovery fit.

Native Scheme membership audits use the same real frame/world mechanics as
the sampler. Independently replay every sampled deal's entire public history,
every forced branch, actor-local request and terminal score in Python. Validate
sampler support/seed stability against existing tests and small census fixtures.
Retain same-world action flips and same-action-pair cross-world contrasts, with
the exact changed holders and divergence history. These are inputs to the next
learning cycle even if the first descriptions do not generalize.

Full artifacts: `~/data/texas-42/kiln-played-v1/whole-game-contrasts-v1`.
No phone change or automatic production restart. This fresh set is consumed
after evaluation; later refinements require new confirmation data.
