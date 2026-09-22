# More playing strength at the present time budget

2026-09-19. **EXPLORATORY DESIGN; no player, GPU benchmark, training run, or
strength experiment was executed for this document.** This is a proposed
quality experiment, separate from the unchanged 100 ms / 28-play / L2 Partner
40/8/2 exact-semantics speed contract in [`../GOAL.md`](../GOAL.md).

Working interpretation: “present time” means the optimized native player's
roughly half-second complete game, rather than the older several-second
reference. This interpretation needs to travel with any proposal until the
user resolves it. The experiment should answer whether a named executed
policy wins more contracts at comparable total latency. Agreement with an
expensive solver, larger samples, a higher level, and GPU throughput are
diagnostics, not substitutes for that answer.

## Current evidence, and the baseline to beat

Authority: `QUICKSTART.md`, `wiki/walt-program.md`,
`wiki/walt-seat-play.md`, `wiki/walt-calculated-evidence.md`, and the current
partnership worktree's `experiments/partnership/PLAYERS.md` and foundation
protocols. The live synthesis sends numerical claims back to their actual
receipts. Everything here remains inside Walt's exploratory fence.

The timing baseline is **native v34**, the latest CPU-work-selected arm in
[`../PROGRESS.md`](../PROGRESS.md): L2 Partner, root/L0/modeled-L1 samples
40/8/2, Fixed/Fixed, voidless inner belief. It is not a newly established
strength champion. Its frozen identity and measured comparisons are in
[`../results/paired-table-arena-v1/`](../results/paired-table-arena-v1/).
The source-copy origin is partnership commit
`95e90444e3b403bf0c009c473b3dd0b13167c71d`.

Read-only aggregation of the saved eight reference games and eight v34
games gives the following context. Quantiles use the nearest-rank definition;
trick shares are shares of summed decision elapsed time.

| Saved timing measure | Original reference | Native v34 |
|---|---:|---:|
| Complete-game median | 4,859.786 ms | 545.752 ms |
| Complete-game range | 2,836.457–7,664.605 ms | 292.202–701.820 ms |
| Nonforced decisions | 152 | 152 |
| Nonforced decision median | 44.566 ms | 6.873 ms |
| Nonforced decision p95 | 1,347.877 ms | 100.018 ms |
| Nonforced decision maximum | 2,302.185 ms | 164.644 ms |
| Trick-1 / trick-2 / trick-3 time shares | 60.36% / 27.16% / 8.99% | 53.40% / 27.91% / 10.44% |

Both sets use the same eight deal identities, but were collected under
different variable background load. They are operating context, not a
controlled latency guarantee. V34's nominal opening work already takes
most of the time; putting all extra investigation late in the hand cannot
answer the user's opening-through-full-game question.

The speed harness measures requested native search without the operational
outer L1 reserve/fallback. It retains 28 actions after make/set is known.
Its complete action/value matches establish equivalence on measured sampled
problems, not improved play. A quality player's resource identity must include
its actual reserve, fallback, timer and policy execution costs.

The strongest relevant current play receipts do **not** establish that a
deeper or more heavily sampled configuration is stronger:

| Completed evidence | What it supports |
|---|---|
| Default L2 Partner vs default L1: 14 wins / 14 losses / 72 ties, 100 fresh paired deals | No demonstrated gain from the partner-level change. Different trajectories occurred on 98/100 deals, so the score tie is not policy equivalence. |
| L2 Partner with voids vs default L2 Partner: 12 / 17 / 71 | No demonstrated gain from this implemented belief change. It also changes the deterministic inner sample stream. |
| Native L1 race vs archived phone: 35/35 fallback-free pairs identical through all 28 plays | A useful procedural phone anchor. The small overall 3/1/46 edge arose with phone fallbacks, not a newly discovered completed-policy improvement. |
| L1 race/refine comparisons; partner race/refine variants | L1 comparisons were inconclusive; tested partner refinement variants crossed the >5% fallback technical gate. |
| Optional partner-rollout review: 0 / 0 / 64 on fresh ordinary pairs | Conditional examples improved and harmed, but no ordinary-game gain. One 52-world stopped prefix preferred the wrong side of a one-world full-census margin. |
| Original L2 All diagnostic on regression deal 420602 | Reproduced L2 Partner's defensive regression (declarer made 36); no broad L2 All vs Partner ranking. At Gran's root All and Partner even chose opposite actions. |

Sources in `/Users/jason/code/texas-42-partnership-launch/experiments/partnership/`:
`campaigns/default-partner-battery/{RESULTS,STRENGTH-ASSESSMENT}.md`,
`campaigns/foundation-battery/RESULTS.md`,
`campaigns/sunshine-rollout-v1/RESULTS.md`, and `REPORT.md`.
The historical L1 victory over mk5 E[Q] in the wiki predates a modeled-cache
purity fix; its opponent and binary are separate historical evidence.

Use v34 L2 Partner as the **incumbent requested by this research task**.
Retain optimized L1 Fixed and L1 Race as named external anchors when the new
player is ready. There is no currently justified single transitive “strongest
player” ordering across these distinct matches and budgets.

## Three plausible quality levers

These are competing candidates for measurement, not promised improvements.
The first two can compose after their separate costs and effects are known.

### A. More independent evaluation of a small frozen policy set

Have CPU search produce a small set containing the incumbent and one or two
lawful alternatives. Freeze the complete continuation rule of each, including
its root action, information keys, partner behavior, field, seed, objective,
and model settings. A root tile without a continuation rule is insufficient.
Allocate additional compute to independent compatible-world evaluation of
these policies, using the same worlds and exogenous random tapes for every
candidate. Keep the candidate builder's sample stream separate from this
evaluation stream. Evaluate make/set differences, not differences between
each candidate's own reported pmake table.

GPU opportunity: a large rectangular array of **candidate × world × fixed
field** forward trajectories, with paired integer outcome reductions. This
is substantially different from launching tiny recursive eight-world L0
jobs. Every world must execute an observation-measurable chooser. Never
maximize independently inside each hidden world. A frozen field still needs
its own legal policy execution at every encountered state.

Start with the incumbent plus one challenger, and completed blocks such as
32 worlds; increase the number of blocks only if end-to-end measurement
permits it. These are proposed engineering sizes, not strength settings.
Measure policy preparation, misses, queue fill, transfer/readback and reduction
inside the decision time. The available ready width and preparation cost are
the central feasibility questions. A large offline rectangle alone does not
establish that a live decision can afford one.

Use simultaneous or anytime-valid paired evidence to justify replacing the
incumbent. Commit a contiguous prefix of a fixed indexed stream, never the
worlds that happened to finish first. An incomplete block changes no estimate.
Merely taking the largest
sample mean among alternatives retains selection bias. On unresolved
comparisons keep the completed incumbent; record an unresolved comparison as
such, not as a certificate that the incumbent is best. This mechanism improves
evaluation fidelity for a declared field and policy set; arena play must test
whether that fidelity is useful.

### B. Pay for a higher response rung only where it can change the decision

Freeze each rung as a best-response problem against a named lower policy,
and preserve the distinction between a sampled best response and an exact
response over the full support. Use cheap bounds to remove settled work and
batch the surviving independent computations. Spend on uncertainty about
important early choices, not on uniformly increasing every modeled count.

For a declared target `Q_F(a)`, valid bounds `L_a <= Q_F(a) <= U_a` permit
removing an action once its upper bound is below an incumbent lower bound.
Canonical first-tile ties require the corresponding strict/non-strict
inequalities; a weak maximizing-member statement alone need not determine
the playable first-tie action. Each bound must identify its support measure,
field, optimization domain, confidence/exact tier, and remaining obligations.

The existing targeted controller already provides relevant mathematics and
code boundaries: field-swap exposure, root-action Lipschitz bounds, admissible
screens, survivor-only work, and a test that refuses exposure escalation
when even a perfect bound cannot prune. Reuse that authority rather than
inventing an ad hoc “looks close” certificate. Only
`RootActionExposureUpper`, not exposure of one frozen policy, may enter a
root-action field-stability screen. A frozen-candidate certificate stays a
frozen-candidate certificate until the competing-continuation bounds close.

GPU opportunity: process all presently admissible candidate/rung/world jobs
in batches, then synchronize only at valid evidence boundaries. The policy
of a modeled seat must be frozen and clock independent; a partial nested
solve aborts the attempt instead of introducing a timeout-selected modeled
move. An outer player may return its labeled completed reserve.

Disproofs to seek: exposure is nearly one at trick 1; bounds are too broad;
screen construction costs as much as the work it avoids; or the allegedly
richer field is a worse model of actual partners/opponents. Higher rungs
need not improve actual make probability and can cycle. The standing intake
does not authorize damping, mixing, or cycle policies without separate math.

### C. Amortize the cost of the lower policy, with a separate strength claim

A closed, observation-keyed compiled policy or a compact distilled lower
policy may make the large fixed-policy rectangles in A affordable. Freeze
the lower policy before evaluation. The upper search remains a best response
to that declared policy; approximation to the teacher does not imply equal
decisions, equal values, or equal strength against real players.

Exact compilation needs full key coverage on the evaluated continuation
domain, or a specified lazy miss route whose generation cost is charged.
Distillation requires separate training/development/evaluation deals and
lawful inputs, with legality, missing-key and out-of-domain behavior fixed
before the test. Teacher agreement is only a secondary diagnostic; the
primary target remains paired contract outcomes. Never allow hidden-world
features into the live chooser merely because they are available to a teacher.

This is the longest-horizon option. Current full-game measurements found
negligible reuse across successive real decisions and very low hot L0 hit
rates; they do not support assuming a cheap cache will amortize most online
work. Learned generalization is a different hypothesis. No models or training
should start until A's actual policy-evaluation cost shows that this expense
would address the bottleneck.

## The fixed-time strength protocol

### Freeze the executed policies and the time claim

Freeze source and executable hashes, compiler features, all modeled levels,
sample/selection rules, lawful keys, seed-domain rules, reserve/fallback
policy, CPU threads, GPU device/runtime, and persistent-resource mode.
Choose one candidate using development data; nominate it before unlocking
the test panel. This prevents a test-set winner chosen from many levers.

“Similar time” should be a predeclared measured criterion, for example
candidate mean **and p95** full-game elapsed time at most 1.10 times incumbent
under matched isolated execution. The 10% tolerance is a proposal, not an
existing agreement. Also publish median, p90, p99, maximum, nonforced
decision quantiles, and time by trick and role. Report startup separately
while including all per-game preparation and device work. Small panels have
coarse tails; report order statistics and uncertainty instead of precision
they cannot support.

Use the same explicit hard execution ceiling for both policies, with a
completed reserve retained before investigation. A provisional operational
ceiling is 1 second for all-four-seat selfplay, and 500 ms per partnership
over its 14 decisions in a mixed game. This is an upper safety envelope;
spending it on every game would fail the roughly-half-second comparable-time
criterion. Confirm or revise the ceiling using **latency-only development
calibration before** outcome evaluation. A constant per-move cap is a different
resource policy and must not be called a complete-game cap.

Retain an uncapped-completed incumbent anchor during development. If the
new ceiling changes its choices or creates frequent fallbacks, explicitly
name the capped incumbent and compare against the completed anchor as well;
do not manufacture a quality improvement by weakening the reference.
In the fresh run, all valid deadline fallbacks remain scored. Report every
reserve, unresolved retention, legal fallback, refusal, overrun, worker fault,
and incomplete pair. No reroll of a valid slow or losing decision is allowed.

Use exclusive CPU/GPU ownership for single-game latency. Rotate AB/BA execution
order across deal blocks; include cold and persistent resources as separate
series. A throughput run with multiple games is a separately labeled result.
An external watchdog can preserve progress but is not a subsecond scheduler.

### Paired full games and coverage

For each frozen complete physical deal and contract, play all 28 legal moves:

1. Candidate A controls both declaring seats; incumbent B controls both defenders.
2. Keep hands, bidder, opening leader, declaration, and bid fixed; swap A/B partnerships.

Let `mA` and `mB` be the declaring make indicators. The independent deal's
score is `D = mA - mB`, with values -1, 0, +1. Report W/L/T, `mean(D)`,
comparative contract fraction `(1 + mean(D))/2`, and declaring-make/defending-set
rates separately. Both-set and both-make pairs tie. Points are explanatory
diagnostics, never tie breakers. This measures play after a contract; no
auction-strength claim follows.

For the general opening instrument, propose stratified bid-30 coverage of
all **4 bidder seats × 9 declarations**, with independent physical deals in
each cell and the same predeclared weighting in both arms. Assign contracts
independently of candidate outputs. This broad fixed-contract population is
different from the existing longest-pip-trump fixture population; report it
as such. Preserve the original sixes/bidder-0 timing panel as a compatibility
anchor, not as the only quality population. Higher bids belong in a separately
declared extension, not an opportunistic favorable subgroup.

Physical seat rotations of the same deal can audit symmetry, but are repeated
measurements of one deal, not independent new hands. The same applies to
multiple hidden completions of one focal opening hand. If those instruments
are used, keep the entire base deal/focal hand as a cluster. Opening, middle,
and late witness analyses accompany the full-game result; none replace it.

Use independent seed namespaces for physical deals, candidate construction,
candidate evaluation, and policy randomness. Common random numbers couple
only matched exogenous quantities. They must not let a policy seed encode
the referee's actual hidden deal. Once trajectories split, preserve each
policy's complete continuation rather than comparing later actions as if
they came from the same information state.

### Confidence, stopping, and promotion

The smallest balanced development measurement is **72 fresh physical deals /
144 games** against one nominated candidate: two independent deals in each
seat/declaration cell. This can expose an obvious regression, rarity of
changed outcomes, fallback cost, and useful/harmful witnesses. It cannot
settle a small general advantage. Keep these deals out of subsequent promotion
evidence after examining them.

After development, use one fixed untouched panel, initially **288 deals /
576 games** (eight per cell), for an exploratory effect estimate. Select
any eventual larger confirmatory sample size before inspecting those larger
outcomes, using only development variance and a chosen minimum useful edge.
The old ~28% discordant-pair rate suggests, purely for planning, that roughly
1,400 independent pairs may be needed for 80% power at a two-sided 5% level
to detect a two-percentage-point comparative contract edge; changed fields
and contract populations can change this substantially. A 100-pair dead heat
does not establish equivalence.

Use a predeclared paired, stratified deal-cluster analysis. Publish a 95%
interval for the weighted mean difference, with the method stated and an
explicit small-sample/degenerate-data guard. A stratified cluster bootstrap
is an approximate descriptive interval, not an exact or anytime guarantee.
For a conservative distribution-free check, also report the fixed-N
Hoeffding interval for `X=(1+D)/2` in [0,1]; its half-width is
`sqrt(log(2/alpha)/(2N))` for equal-weight independent deal units. It is wide
but remains honest when all observed pairs tie. Neither repeated bootstrap
inspection nor the existing rough ±2 SE report supplies optional-stopping
validity. If sequential promotion is wanted, freeze a valid confidence-sequence
or alpha-spending rule before play.

Promotion requires a fresh positive quality result under its declared analysis,
the measured comparable-time gate, and acceptable fallback/overrun behavior.
A latency pass with an interval crossing zero is “quality unresolved.” A
conditional evaluator gain with no arena gain remains conditional evidence.
Keep technical stops separate from outcome stops and include the planned-order
prefix and all missingness in the report; never select completed pairs by speed.

If multiple candidates enter a confirmatory run, predeclare multiplicity
handling or reserve a second untouched confirmation panel for the selected
winner. Multiple opponents give a matchup matrix, not an assumed transitive
rating. Reused deal panels are paired across comparisons, not new independent
evidence each time.

## Existing harness entry points and missing integration

Recovered from source; **these commands were not run**. From the partnership
worktree, the existing ordinary matched-player path is:

```sh
python3 experiments/partnership/match.py players --all
python3 experiments/partnership/match.py init experiments/partnership/campaigns/NEW-RUN --a l2-partner-default --b l1-default --start NEW-SEED --count 100 --threads 6
python3 experiments/partnership/packet/texas42-partnership-launch-v0.1/tools/run_capped.py --seconds 295 --output-dir experiments/partnership/runs/NEW-SLICE -- python3 experiments/partnership/pool.py experiments/partnership/campaigns/NEW-RUN --workers 1 --seconds 270
python3 experiments/partnership/verify_campaign.py experiments/partnership/campaigns/NEW-RUN
python3 experiments/partnership/match.py report experiments/partnership/campaigns/NEW-RUN
```

`NEW-RUN`, `NEW-SLICE`, and `NEW-SEED` are placeholders. The commands above
only illustrate current capabilities; the stock presets retain their historical
14-second **per-move** wrapper and are not the proposed equal-time run.
`--players PATH` can supply a custom frozen catalog. `--panel worlds` and
`--worlds-per-hand N` support grouped completion panels. `campaign.py stop PATH`
and `campaign.py resume PATH` preserve checkpoints.

Required narrow seams before this design is runnable:

- Bind independent incumbent/candidate artifacts to the existing referee and
  persistent-worker interface; preserve own-original-hand/public-only requests.
- Add an explicit whole-game/partnership bank and complete reserve accounting;
  the current `Player.budget_ms` is per move and has a 100 ms minimum.
- Add a frozen manifest of balanced contracts/deals. Current `fixture()` fixes
  bid 30 and chooses the strongest pip-trump/bidder across seats; it covers
  declarations 0–6, not the proposed balanced nine-declaration panel.
- Retain per-decision and whole-game timing, completion/fallback routes,
  device/preparation time, panel identity, full play traces and independent
  replay verification; add the predeclared clustered statistical report.

The existing `full-game-speed/tools/run_paired.py` is a balanced-order
**same-policy selfplay timing/parity** instrument. Its equality gate compares
exact root values/choices. It does not seat different policies against one
another and cannot provide the proposed strength score without an adapter.

## The next measurement that actually answers quality

Once one candidate has a complete legal execution path, compare it with the
frozen incumbent on the 72-deal balanced development panel, retaining all
144 full continuations and resource costs. Independently time the same two
policies under isolated all-four-seat selfplay to check the promised total
latency. Do not spend the first quality budget on hundreds of scalar/GPU
action-vector agreement checks after parity is already established.

For every changed contract, preserve the first shared-state disagreement and
both later partnership continuations. Use a separate common-world evaluator
only to explain how the outcome changed, clearly naming its field. The first
question is how many contracts improve or worsen within comparable time;
the second is which lawful role/order/partner mechanism produced those changes.
That is enough to decide whether the next investment should be a fresh larger
play panel, better policy candidates, cheaper policy execution, or stopping
an unhelpful lever.
