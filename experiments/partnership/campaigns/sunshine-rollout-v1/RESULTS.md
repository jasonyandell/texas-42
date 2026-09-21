# A fast partnership check, with a measured sampling mistake

2026-09-13. **Exploratory.** The optional `l1-partner-rollout` player now asks
whether offering or withholding count helps its partnership make 30, then
compares actual completed L1 continuations within a bounded allowance. It uses
the same native L1 evaluator as the unified player. This is a working instance
of the sunshine plan: baseline, public opportunity detector, targeted
investigation, and hindsight measurement. It addresses one late-game skill;
general partnership strength remains open.

The development panel improved substantially. The untouched panel was small
and mixed: one useful correction, one smaller harm, and no make/set changes in
ordinary games. Keep the candidate available for research; these results do
not warrant replacing L1 default.

## What was frozen

The [protocol](PROTOCOL.md) preceded candidate measurement.
[freeze.json](freeze.json) pins code/binaries and the fresh match after
development. Fixed L1 40/8, voidless inner samples, bid 30; uniform mechanical
outer support respecting public voids, capped at 400 worlds. Up to 64 shuffled
worlds, all legal root moves on each, completed L1 afterward at all seats.
Only fully compared worlds count. At least eight worlds or a smaller full
census; strict observed improvement, baseline ties retained. Up to 500 ms
extra inside the existing 14-second ceiling. No tuning during fresh evaluation.

Unlike the older reviewer, this check can question an offer already selected
by L1. It does not reward count as an independent feature: make/set decides.
Every future chooser sees only its own original hand and the public record.
The full world belongs to the simulator and examiner, never the chooser.

## Conditional decision quality

Every row below compares the selected actions against a full compatible-world
census under **completed default L1 continuations**. These are model-relative
values, not true human-table make probabilities. All declaring query matches
are included, even when every action ties. Source deals, not hidden-world
completions, are the independent grouping units.

| Measurement | Development | Fresh |
|---|---:|---:|
| Matched declaring roots | 143 | 25 |
| Source deals with matched roots | 56 | 20 |
| L1 best-valued choices | 129 | 24 |
| Candidate best-valued choices | 137 | 24 |
| Improved / harmed / equal-value roots | 11 / 2 / 130 | 1 / 1 / 23 |
| Mean root regret, L1 | 0.6686 percentage points | 0.3200 percentage points |
| Mean root regret, candidate | 0.0608 percentage points | 0.01905 percentage points |
| Census checks completed by candidate | 107 | 20 |

Regret is best available make probability minus selected-action make
probability. Mean changes are **+0.6078 pp** in development and **+0.3010 pp**
fresh when roots receive equal weight. Equal source-deal weighting gives
**+0.7621 pp** and **+0.1762 pp**, respectively. Exact rational values and every
source group's result are in [summary.json](summary.json); all 168 root
requests, choices, full reference action counts, and sampled comparisons are
in [roots.json](roots.json).

The fresh gym scanned 336 eligible late positions from the 64 new baseline
arms: 40 exceeded the support cap, 264 had no query match, and 32 matched
(25 declaring, seven defending). Ten declaring positions had strict outcome
differences. The candidate measurements include all 25 declaring matches,
not only those ten. The [specification](fresh-gym.json) preserves the unchanged
Scheme question and binds its fresh sources.

These 25 roots supply only **two changed decisions from two source deals**.
A lower mean regret here is encouraging evidence about this finite panel,
not a settled estimate of the skill's general benefit.

## Two fresh decisions, including the mistake

Both full examples are [checked in](examples/catalog.json), with every legal
action's complete census and trajectories.

- **Useful offer, seed 99210014, three own tiles:** L1 chose 6–2; the candidate
  offered 3–2 to its currently winning partner. Under the frozen continuation,
  make count increased from **10/50 to 14/50**, with four saves and no losses
  on paired worlds. The candidate completed the entire census.
- **Harmful withholding, seed 99210001, three own tiles:** L1 offered 5–0;
  the candidate kept it and played 6–5. At the deadline, 52 complete paired
  worlds favored withholding **34–33**. Full support favored offering
  **140/210 versus 139/210**: four saves but five losses from the intervention.

The second example isolates a useful failure mode. Replaying the same shuffled
stream after measurement gives 41–40 **for offering** at 64 worlds. Both the
64-world prefix and the full census would keep L1; the budgeted prefix happened
to stop while the small difference pointed the other way. This is why the
candidate's sampled choice is explicitly a guess. No subsequent threshold or
sample-budget tuning was applied to the player using this outcome.

The natural next investigation is how much paired evidence should justify
changing the baseline, and whether to concentrate remaining time on close
comparisons. Simply suppressing small gains could also erase legitimate rare
help. Keep the good offer and this counterexample together when testing that
next idea.

## Ordinary games and cost

64 fresh mirrored deals, seeds **99210000–99210063**, produced **0 wins / 0
losses / 64 ties** for the candidate. Both-make and both-set pairs tie; points
never break ties. All 128 games completed, with no player fallbacks. There
were 24 active reviews: 22 retained L1, one changed it, and one exceeded its
outer allowance and kept the baseline. The changed move was the useful 3–2 offer above;
the actual source world did not change make/set. This panel supplies no
ordinary-game strength gain or equivalence claim.

The fresh gym run changed one additional move: that review timed out in the
arena, whereas the root study received its 52-world prefix before the outer
limit. The player is budget-sensitive; repeating the same input under different
load can change the sampled prefix or whether the worker returns in time.
Completed records remain authoritative when resuming an experiment.

| Measured time | Development roots | Fresh roots |
|---|---:|---:|
| Mean candidate decision | 117 ms | 128 ms |
| Mean investigation itself | 112 ms | 123 ms |
| Maximum investigation | 473 ms | 469 ms |

The root study invokes baseline first, then candidate through the warmed native
session. Its baseline/candidate timing difference is therefore not used as an
unbiased overhead estimate; the investigation phase above includes its own
worker startup. Ordinary arena means, including forced moves and all wrapper
work, were **183 ms for L1 and 185 ms for the candidate**. These are local
ten-game-pool timings, not guaranteed latency or a precise causal overhead.

We interrupted the match with SIGINT after 64 complete games, cleared its stop
marker using the normal resume command, and finished all 128. All **31 already
committed pair records** at interruption remained byte-identical. Per-move
checkpoints also retained progress in incomplete games. Every workload used
an external 295-second watchdog; the two active match slices took about 36
seconds each, and the fresh gym took about 30 seconds.

## Verification and reproducibility

- 89 Python tests passed, including six new wrapper/configuration/refusal
  tests. The six were rerun after strengthening the configuration cases.
- 38 targeted native tests passed: new rollout tests plus the existing
  reviewer, partnership evaluator, and unified-solver checks.
- Before the fresh experiment, the adapter reproduced all **930 trajectories
  and 4,604 unique decisions** in three saved examples by exact own/public
  input. Independent Python rules replay verified every retained trace prefix,
  banked score, and settled make/set result.
- The two fresh examples added **780 trajectories and 3,221 unique decisions**,
  including a different bidder orientation. Full native values and individual
  choices agreed with the separately executed deployed gym.

A final code review corrected failure reporting after the frozen measurements:
an infeasible continuation frame now propagates as a worker error, preserving
the baseline, instead of being mislabeled a deadline. No settings or successful
decision calculations changed. The 20 directly relevant native tests and both
complete audit panels passed again after this correction. Measured and release
binary identities are retained separately in the summary; rebuilt binary
hashes are not silently substituted for those of the measured experiment.

The [player guide](../../PARTNER-ROLLOUT.md) explains operation and limits.
`rollout_study.py audit` verifies a portable gallery against a saved deployed
generation receipt; `measure` evaluates all declaring matched roots from a
discovery directory. Both preserve the seven-field pupil input boundary.
`publish.py` regenerates the compact evidence and checks case hashes, exact
regret arithmetic, fresh freeze identity, and interrupted-record preservation.

Raw manifests, matched games, all root values, per-decision caches, audit
trajectories, and watchdog receipts remain at
`/Users/jason/data/texas-42/sunshine-rollout-v1/`; important file hashes and
workload commands are checked into `summary.json`. To regenerate elsewhere,
recreate the frozen match, bind the equivalent source paths in `fresh-gym.json`,
generate the deployed-L1 gym, and measure its unfiltered declaring matches.
Changed code/binary identities require a new run namespace; they never silently
resume an old frozen evaluation.
