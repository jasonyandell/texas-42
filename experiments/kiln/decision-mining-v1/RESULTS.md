# A working contrast rig, thirteen positions to study, no new move rule

Exploratory, 2026-09-18. **The loop now runs from actual played games through
Scheme descriptions, paired ordinary-player continuations, frozen selection
and fresh evaluation. It found genuine model-relative missed opportunities,
but none of this small action grammar's rules improved even development play.**
The correct selected fragment was therefore the empty one: retain Walt.

[Protocol](../DECISION-MINING-V1.md) · [runner](../decision_mine.py) ·
[publisher](../decision_mine_publish.py) · [frozen fit](fit.json) ·
[fresh result](result.json) · [compact summary](summary.json).

## What was assessed

Both panels contain 120 declaring-team decisions with **two dominoes left**,
two legal choices, and an unsettled bid30 contract. Twelve coordinates per
source deal were selected by a fixed hash, before assessing alternatives and
without filtering on losses, original choices or threat matches.

- Development: ten old source deals 420600..420609, trial zero, all four
  bidder hands and nine declarations. There were 319 eligible coordinates.
- Fresh: ten new source deals 420635..420644, the same 360-game design, producing
  304 eligible coordinates. Their alternative-action labels were opened only
  after the selected fragment was frozen and committed.
- Each position was assessed under two policy seeds, giving **240 comparisons
  per panel**, not 240 independent positions. All compatible hidden hands were
  enumerated (1..90 worlds), with the same worlds/seed for both actions.
- After the first move is forced, all four seats use the actual shared
  `walt_player::decide`, 40 worlds, partner review, 14-second budget. Every call
  contains only that actor's original hand and public history. Identical calls
  within an assessment reuse their first complete response.

Values are success probabilities under the **uniform mechanical support and
these recorded ordinary continuations**. That prior omits behavioral inference
from earlier plays. Full support removes outer-world sampling error for this
declared comparison; it does not make Walt or this belief model optimal.

## Could a different move help?

| Measurement | Development | Fresh |
|---|---:|---:|
| Position/seed comparisons | 240 | 240 |
| Both moves have different make probabilities | 116 | 89 |
| Walt chose the lower-valued move | **7** | **10** |
| Distinct positions with a miss under either seed | **6** | **7** |
| Mean baseline regret, all comparisons | 0.185 pp | 0.228 pp |

Walt was best or tied in 230/240 fresh comparisons. There are useful exceptions,
but little average room for an indiscriminate two-tile override in this panel.
The exceptions remain worth investigating; rarity does not erase their value
as exercises or their possible importance to a human partner.

All **17 missed position/seed cases across 13 distinct positions** are published
with paired witnesses: [development](train-misses.json),
[fresh](validation-misses.json). Each retains the whole information set,
both action values, the original model response, and a world where the
alternative helps; a world where it harms is included whenever one exists.
The full action-by-world table and every original response remain in the durable
assessment databases, including all ties and controls.

The largest fresh gap illustrates why paired worlds matter. On source deal
420639, policy-seed replicate 1, blanks are trump and the bidder leads with
**5-0 or 6-4** remaining. Walt selected 5-0:

| Lead | Walt's root model | Full-support deployed continuations |
|---|---:|---:|
| 5-0 | 3/10 = 30% | 4/19 = 21.05% |
| 6-4 | 11/40 = 27.5% | 6/19 = 31.58% |

Changing only that lead helped in three worlds, harmed in one, and tied in
fifteen. The net gain is **2/19 = 10.53 percentage points** under this particular
seed/continuation contract. A favorable actual world alone would have hidden
the harmful counterexample. This is a saved exercise, not a general instruction
to lead 6-4 or to withhold trump.

## Did a shared description transfer?

The action grammar uses nine mechanical properties: called/not called,
double/not double, count 0/5/10, and boss/not boss in the context the action
would lead. It considers witnessed single properties and pairs, expressed as
Viewer-only Schemes. Physical tile IDs, action score bonuses, attention models
and manually written count-offer tactics are absent.

Each action expression may apply always, or only when the integrated presence
of either individually named opponent's live top trump reaches 1/3 or 2/3.
It changes the baseline only when exactly one legal action matches.
This produced **114 witnessed query/gate candidates**. Every candidate's mean
development gain was nonpositive; several zero-gain candidates changed no moves
or only tied moves. None met the frozen support and improvement criteria.

The selected policy is consequently **the baseline**, with zero fresh changes
by construction. Its zero gain and zero interval are not evidence for a new
skill. The limited grammar lacks richer partner/order/continuation relationships;
failure here does not establish that such relationships cannot be learned.
The [fit](fit.json) retains all candidates and their harms rather than selecting
a pleasing retrospective example.

## Did the learned threat locate mistakes?

The opening opponent-top-trump Schemes were re-evaluated at the current state,
an explicitly new use of those learned descriptions. A gate tests the **maximum
of their two individual support presences**, not the actual hidden holder.

| Gate | Development misses / assessments | Fresh misses / assessments |
|---|---:|---:|
| Below 1/3 | 5/148 | 2/112 |
| At least 1/3 | 2/92 | 8/128 |
| At least 2/3 | 0/18 | 0/26 |

The moderate gate enriched misses on fresh hands but not development. The
strong gate found none in either panel. This does **not** establish a reliable
"think harder here" detector. A condition associated with losing can describe
an unfavorable situation without identifying a fixable choice.

The separate sample-efficiency diagnostic is more encouraging, though still
limited: partitioning worlds by whether either opponent holds the live top
trump explained **10.27% / 9.87%** of the variation in the *difference between
the two actions' outcomes*, on development/fresh panels respectively.
These are retrospective finite-support variance decompositions using both
strata's observed means. They are not measured sample savings or a fitted
allocation procedure. They justify a later controlled estimator experiment,
not a claim that Walt can now use 10% fewer samples.

## Engineering, timing and verification

- New native `kiln-decision-worker` wraps the unchanged shared player. The
  existing full-game producer and phone player were not changed or deployed.
- New source games: **360 in 49.17 seconds**, 18 workers. Development and fresh
  paired panels: **13.82 seconds each**, 10 workers, including first independent
  validation and durable writes. These are separate stage timings, not total
  elapsed implementation/re-audit time and not phone measurements.
- **21,620 complete forced-action continuations**, **144,308 subsequent plays**,
  **107,601 recorded unique-per-assessment decisions**, and **508,070 Scheme
  membership checks** across the two main panels. No worker errors, over-budget
  decisions or fallback routes. Independent audits also checked all 10,080 moves
  in the new source games.
- Seven focused Python tests and one Rust rejection test pass. They cover
  hidden-input rejection, native/Python mechanics and Scheme agreement, corrupt
  outcomes, omitted support worlds, missing assessment jobs, paired harms/ties,
  and a variance-decomposition negative control.
- A deliberate one-second stop saved 15 new assessments with 85 pending;
  resumption finished those 85 and preserved **all 155** already-completed
  receipt hashes. See [resumption audit](resumption-audit.json).
- After completion, the reader gained an additional missing-job check and the
  publication utility was added. Recomputing the frozen fit and fresh result
  returned the exact same content IDs. The original measurement sources and
  binary remain in each external producer archive; this hardening changed no
  choices, values, selected rule, hypotheses or evidence.

Durable raw data:
`/Users/jason/data/texas-42/kiln-played-v1/decision-mining-v1/`.
`train/` and `validation/` contain immutable panels, original source receipts,
producer archives and resumable `assessments.sqlite`; `fresh/` contains the new
complete games. SQLite rows contain compressed full receipts and SHA-256 hashes.
The separate resumption audit is not counted as additional research evidence.

Fit ID: `83798ea0e3f595adc8a4d1f4e8dd893017c82a08ed16ec0cf7c218b789ff5085`.
Fresh result ID: `399511376e51b65713772f0ba053934963c13f5108f488f12ba7b2596326773b`.

## Using the rig again

`decision_mine.py prepare SOURCE OUTPUT --split train|fresh` selects and pins
the declared panel. `run OUTPUT --workers 10 --seconds 60` runs a bounded batch;
repeat unchanged to resume. Use the existing process-group watchdog around
runs. `fit TRAIN FIT.json` freezes selection; a fresh run requires its parent's
`fit.json` and pins that identity before any assessment. `report FRESH FIT.json
RESULT.json` evaluates only the frozen selection; `audit OUTPUT` rechecks every
receipt, support world, Scheme answer and actor-local continuation.
`decision_mine_publish.py CAMPAIGN DESTINATION` reproduces the compact evidence
and concrete counterfactuals from completed assessments.

This v1 recipe deliberately fixes its seeds, domain and grammar. A different
question needs a new protocol/output, not overwritten evidence. Source changes
refuse resumption with a different producer; an exact historical rerun uses the
saved source archive and binary. The fresh panel is now consumed and cannot
be recycled as untouched confirmation for another learned rule.

The next useful material is the **13 concrete misses and their successful,
harmful and tied continuations**. They can guide a richer relational grammar or
an investigation trigger. The present experiment supplies no reason to deploy
one of its simple move overrides.
