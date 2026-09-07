# Compose the partnership query with bid-making

2026-09-07. Exploratory, field-relative exact exercise keys and a first
paired L1/L2 Partner diagnostic. The maintained statement is
**[specs/partnership-bid-making.json](specs/partnership-bid-making.json)**.

## What composes

In the original terminology, a **Scheme** is one typed relational case;
a **Fix** combines cases by union. The executable language uses `(fix ...)`
with one or more `(case ...)` forms. Calling it a Scheme/Fix expression is
appropriate. Clauses within a case are conjoined; separate cases are alternatives.

This specification combines the existing
[offer-count expression](queries/offer-count.scheme) with the bid-making
evaluator and a stricter outcome condition. The expression's two cases return
legal five-count or ten-count plays that leave our currently winning partner
ahead. It reads public state and our own hand only.

Let `A` be all legal plays, `T` the plays returned by that expression, and
`Q(a)` the existing lawful focal best-response make probability after `a`,
under the declared belief and future field. The new statement selects:

```text
our side is declaring
T and A \ T are both nonempty
max Q(a), a in T  >  max Q(b), b in A \ T
```

Its reusable selector is `selection.criterion = query-required`. Every optimal
root choice must therefore match the partnership expression. Comparing against
the **best nonmatching alternative** matters: beating one bad nonmatch while
tying another would not establish that offering count is needed for an optimal
choice. The implementation tests this counterexample explicitly.

The composition happens at the gym-specification layer. Scheme supplies the
action relation; exact evaluation supplies values; the selector compares them.
No solver reward or heuristic count bonus was added, and Scheme itself still
has no solver dependency. The same selector can compose with another action query.

This means **offering count is required for an optimal root choice under this
field**. It does not establish that a hypothetical different partner policy,
or every possible opponent, makes cooperation necessary. Most selected roots
have a strict probability improvement rather than a guaranteed binary flip.
Existing `selection.certain=true` can further select certain success/failure
swings; it is not part of this default collection.

## The first composed family

The default arguments reuse the same recorded-game source, bid 30, trick-5/6
domain, 400-world cap, fixed L1-40/8 teammate and L0-8 opponents. The whole
compatible root belief is retained for grading. The specification embeds the
existing count-offer Fix unchanged and selects declaring-side `query-required`
cases. It contains no coordinate list.

Generation reproduced **30 positions and their complete native keys**, in
**11.075 seconds** on the final implementation. They come from **21 source
deal seeds**: 23 trick-5 positions and seven trick-6 positions. All 30 happen
to have a unique best play; uniqueness was not a selection condition.
One position has a certain make-versus-set swing.

The source scan still accounts for all 1,929 eligible coordinates. It graded
206 count-offer matches across both sides, skipped 245 above-cap coordinates,
and saved 1,478 nonmatches. The generated discovery record retains these
denominators and the cases that fail the final outcome/side criterion.
No measurement failed. The original broader bid-making family remains unchanged.

## L1 versus L2 Partner on identical questions

All 30 generated exercises were presented to `l1-default` and
`l2-partner-default`, with existing default fixed-search settings, seed 420600,
and the 14-second move ceiling. Each pupil received only the seven own/public
request fields. It received neither the query nor the teacher's labels or hands.

| Player | Optimal root choices | Mean make-probability regret | Mean decision seconds |
|---|---:|---:|---:|
| Default L1 | 24/30 (80.0%) | 0.8007 percentage points | 0.0860 |
| Default L2 Partner | 26/30 (86.7%) | 0.4673 percentage points | 0.0883 |

All 60 decisions completed without a fallback or over-budget result. The
ten-worker run, including pre-run key audits, took **1.458 seconds**. Latencies
describe this short endgame exam under parallel load, not opening decisions.

L2 improved three root choices, worsened one, and agreed on the other 26.
Its mean regret reduction is exactly **1/300** make probability, or one-third
of a percentage point. The compared decisions are:

| Exercise | L1 choice/regret | L2 Partner choice/regret |
|---|---|---|
| advantage-06 | 1–1 / 11/105 | 5–0 / 0 |
| advantage-11 | 5–0 / 0 | 5–3 / 1/24 |
| advantage-29 | 6–1 / 1/35 | 6–4 / 0 |
| advantage-30 | 6–0 / 1/120 | 6–4 / 0 |

Both players miss the same three other exercises: advantage-20, advantage-22,
and advantage-27. These and the four disagreements are concrete candidates for
the next analysis. Both solve advantage-09, the certain make/set case: with
25 banked points, give 5–0 to the currently winning partner rather than
overtake with the remaining trump 3–1. The answer values are 3/3 versus 0/3.

This is a selected 30-position diagnostic with a specified teacher field,
not a general playing-strength estimate. Multiple coordinates share source
deals; hidden completions are not independent games. Root regret prices the
pupil's first choice followed by optimal lawful focal continuation, rather
than grading its complete future policy. No uncertainty interval or universal
partnership-strength claim is inferred from four disagreements.

## Reproduce, inspect, and extend

```sh
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-partnership-generation -- python3 experiments/partnership/gym.py generate --spec walt/gym/specs/partnership-bid-making.json --output /tmp/my-partnership-gym --workers 10 --seconds 240
python3 experiments/partnership/gym.py show advantage-09 --gallery /tmp/my-partnership-gym
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/my-partnership-pupils -- python3 experiments/partnership/gym.py run --gallery /tmp/my-partnership-gym --output /tmp/my-partnership-pupils --workers 10 --seconds 240 --players l1-default l2-partner-default
python3 experiments/partnership/gym.py report --gallery /tmp/my-partnership-gym --results /tmp/my-partnership-pupils
```

The saved [benchmark report](benchmarks/partnership-bid-making-v1/report.md),
[exact summary and coordinate map](benchmarks/partnership-bid-making-v1/summary.json),
manifest, and all 60 pupil responses are checked in. Full exercise replays
remain generated artifacts, reproducible from the statement. To read the
saved benchmark against a fresh reproduction, point `--results` at
`walt/gym/benchmarks/partnership-bid-making-v1`.

New pupil manifests identify the exam by its questions and full answer keys;
regeneration timing and provenance metadata cannot make the same exam appear
different. Earlier manifests retain their original full-artifact identity
contract. Both independently generated versions of this 30-position exam
were checked against the new saved benchmark identity.

Twenty-five focused Python tests passed, including best-alternative strictness,
matching and nonmatching ties, empty/full target rejection, unchanged query
composition, portable regenerated exam identity, and prior replay/resume gates.
Every generated case was independently audited before publication and again
before presenting it to pupils. The native solver and player were unchanged;
full repository CI remains waived for the session.
