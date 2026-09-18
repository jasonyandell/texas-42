# Fresh mining replication and measurement efficiency

Exploratory, 2026-09-18. **Both predeclared primary patterns replicated on fresh
hands, and the learned top-trump pattern modestly improved measurement efficiency.**
This supports continued investment in an outcome-first Scheme mining rig. It does
not yet establish stronger partnership play or faster Walt-internal planning.

## Frozen experiment

[Protocol](../MINING-PROBE-V2.md), [pre-label variance extension](../MINING-PROBE-V2-CV.md),
[engineering correction](../MINING-V2-AMENDMENTS.md).

The earlier discovery corpus supplied 2,160 games from 15 source deals / four
trials. Its same 64 losing world examples generated 411 distinct relational
descriptions by replacing tile identities with existential roles, retaining
mechanical properties and optional beating relationships. Four candidates were
selected from 223 eligible descriptions using discovery data alone. The two
highest describe opponents holding the live top trump; the other two describe
opponents holding a double that beats an own-hand trump lead.

The fresh campaign used **ten new source deals, 40 bidder hands, all nine
declarations, eight trials each: 2,880 games / 320 hidden completions**. Seeds
420625..420634 were outside the discovery corpus. Declarations share each
completion and policy seed. No weak cells were screened, no worlds were selected
by a candidate query, and the fixed sample was completed before evaluation.

The player was the same deployed bid30 profile and pinned binary
`91386e6e8d9f2f934c6935a8718d20b1bf46d72e1f274608e22cfe2114af833a`.
No overruns, fallbacks, or worker errors occurred. Two resumable firings completed
in a combined 907.13 seconds while the original bidding campaign also ran.

The primary queries and coefficients were frozen and committed before fresh
outcome analysis. A pre-evaluation complexity-metadata fix did not change any
selected query, its order, or either coefficient; both fits remain preserved.

## Fresh outcome associations

`before` is the last opponent in cyclic order relative to the opening bidder;
`after` is the first opponent. Values below are failure-rate excess above the
matched mixture of hand/declaration cell baselines. They are **not player gains**.

| Query | Matching games | Matching completions | Failure rate | Cell baseline | Excess |
|---|---:|---:|---:|---:|---:|
| **Primary: before holds 5-5** | 747 | 83 | 74.70% | 70.05% | **+4.65 pp** |
| **Primary: before holds top trump** | 653 | 294 | 83.92% | 73.79% | **+10.13 pp** |
| Secondary: after holds top trump | 607 | 284 | 83.03% | 73.95% | +9.08 pp |
| Secondary: before holds double beating own trump lead | 687 | 291 | 80.20% | 72.33% | +7.88 pp |
| Secondary: after holds double beating own trump lead | 642 | 281 | 80.06% | 72.66% | +7.40 pp |

Both primaries matched all ten source-deal groups and passed the predeclared
gate: >=3 pp excess, >=20 matching completions, >=5 source deals, and one-sided
permutation p<=.025 (Bonferroni for two primary tests). Both permutation values
were 1/2000, the minimum resolution of this 1,999-draw check. These are not zero
probabilities or probabilities that the hypotheses are true. The 13-query
max-statistic values were .0125 for double-five and .0005 for top trump.

Permutations move trial outcome vectors together across declarations, within
each bidder hand. They preserve both shared-declaration dependence and cell
difficulty. Matching-completion counts for relational queries count a completion
once if it matches under any declaration; 653 games are not 653 independent worlds.

All eight original literal patterns were retained in the evaluation. The old
strongest training candidate (after 4-4 AND mate 6-2) had **-0.48 pp** excess on
the fresh corpus. The earlier interesting pair (mate 6-2 AND before 5-5) had
+5.93 pp, but it was not a primary here and its 13-query adjusted p=.214.
Another secondary literal pattern had adjusted p=.0275 but only 15 matching
completions, below the registered coverage gate. These are all in
[result.json](result.json); none were removed or silently promoted.

## Does one learned pattern save measurement work?

The top-trump primary identifies possession of a single tile. When that tile is
unseen, each hidden chair has probability 1/3 under the campaign's uniform opening
target. When the viewer holds it or there is no called suit, the event has
probability zero. This gives an exact event prevalence without enumerating hands.

Using discovery games only, freeze beta=56/311 and correct each failure indicator
Y using `Z = Y - beta*(X-p)`. Because `E[X]=p`, this preserves the target mean for
fixed beta. No fresh-label fitting, extra games, changed proposal, or clipping is
involved. The double-five primary receives its own discovery coefficient 50/549.

| Frozen correction | Variance reduction, all 360 fresh cells | Descriptive 95% interval |
|---|---:|---:|
| Double-five ownership | 0.53% | -0.13% to 1.14% |
| **Learned top-trump ownership** | **2.34%** | **1.66% to 3.06%** |

On the 240 cells where top-trump ownership is uncertain, its reduction was 3.70%
(descriptive interval 2.64%–4.84%). The other cells require no correction.
The double-five efficiency result is inconclusive despite its replicated outcome
association: prediction alone does not guarantee a useful shortcut.

These are average within-cell sample-variance comparisons on the fixed eight-game
sample. The interval is a 1,999-draw bootstrap over only ten source-deal groups,
not an exact coverage guarantee. Lower variance suggests modest potential sample
savings for this fixed-player measurement; it is not a measured wall-time gain,
a new bid-selection rule, or a speedup of Walt's internal joint-sample planning.
Full rational arithmetic, per-cell results, and controls: [cv-result.json](cv-result.json).

## Reusable mining components now present

The [catalog](catalog.json) keeps all 13 queries, their sources, provenance,
coverage, original test results, and explicit evidence status. Only the two
predeclared primary successes receive `replicated-outcome-association`; all
queries remain **analysis-only**. Strong secondary observations remain visible.
The emitted `.scheme` files use the real Scheme engine, not an imitation DSL.

`scheme_mine.py` separates fit, fresh evaluation, and publication. Artifacts are
content-identified and refuse replacement with different contents. The full
bounded grammar and original witnesses remain in [fit.json](fit.json).
`scheme_mine_cv.py` separately freezes coefficients and measures efficiency.
The existing actual-play runner supplies atomic game receipts and resumption.
These are the first components of a mining rig, not yet a general automatic
discovery service or a late-game trajectory learner.

```sh
# Python 3.10+; use a NEW output directory for a changed protocol.
python3.12 experiments/kiln/scheme_mine.py fit V1_CORPUS --output ANALYSIS
python3.12 experiments/kiln/scheme_mine_cv.py fit --corpus V1_CORPUS --output ANALYSIS
# Only after the complete fixed fresh batch exists:
python3.12 experiments/kiln/scheme_mine.py evaluate FRESH_CAMPAIGN --output ANALYSIS
python3.12 experiments/kiln/scheme_mine_cv.py evaluate --output ANALYSIS
python3.12 experiments/kiln/scheme_mine.py publish ANALYSIS --output CATALOG
```

V2 deliberately checks its fixed protocol bounds and refuses a different dataset.
Changing a research question calls for a new declared protocol, not overwriting
an earlier experiment's evidence.

## Verification and durability

- The actual Scheme engine checked **1,012,464** memberships: full generated
  library on source examples under every declaration/chair rotation, selected
  training queries, and all fresh queries. Zero mismatches.
- All **80,640 plays** in the fresh corpus passed the independent actor-only
  request, legality, scoring, receipt-hash and producer audit.
- Eight focused tests cover generalization witnesses, rotation and label
  independence, atom accounting, immutable artifacts, known event probabilities,
  exact zero-correction behavior, and zero variance with a perfect control.
- Full games, models, both fits, source/binary snapshots, audits and logs are in
  `/Users/jason/data/texas-42/kiln-played-v1/mining-v2/`, outside git and included
  in the campaign's existing backup scope. Tracked files are compact evidence,
  runnable queries, protocols and reusable tools.

Fresh result ID:
`52e8db032439934d7912b4891e2ba81d31d2c2dc7e3f9343e6bd22244a69462e`.
Efficiency result ID:
`8bfa65719f7e1c6c8008ab852312a89ce0ba54f8a812d3842c06a2ba36f05b93`.

## The next sunshine connection

The campaign has become a usable source of measured, reproducible regularities.
The next useful extension is to mine **decisions with consequences**: find a
learned condition, collect matching plays and controls, then let the continuation
gym test whether an alternative action actually helps. Vary policy seeds and
partner behavior under controlled comparisons. Some predictive patterns may
describe unavoidable difficulty; those still help measurement but cannot justify
a move override. This keeps the rig aimed at better, affordable partnership play.
