# First sunshine cycle: a useful diagnostic, no demonstrated strength gain

2026-09-13. Exploratory evidence under the [declared protocol](PROTOCOL.md).
The optional `l1-partner-count-review` fixes selected missed count offers at
small average measured cost. It changed no decisions in the final 100 fresh
paired deals, so that panel supplies no evidence of a practical strength gain.
Keep default L1 unchanged and retain the review as an experimental preset.

The most specific partnership finding is a post-hoc controlled comparison:
in two selected positions, changing only the checker's modeled partner reverses
the action preference. Four other selected corrections also occur under the
simpler partner model. These are distinguishable mechanisms worth preserving.

## What was built

The player first finishes its ordinary L1 decision. A public detector identifies
a narrow concern: with at most three own dominoes, could an overlooked count
play leave its currently winning partner ahead? The maintained Scheme expression
supplies the alternatives. A bounded check compares them with the baseline,
using the full uniform mechanical root support and lawful focal continuations
against an L1-40/8 partner and L0-8 opponents. Only a strict modeled make gain
changes the move. Ties and unfinished checks keep the completed L1 choice.

The extra allowance is 250 ms within the existing 14-second overall deadline,
with 400-world and 100,000-node caps. The intervention is declaring-side, bid-30,
and only runs after a completed primary decision. It adds no count bonus.
The [guide](../../PARTNER-REVIEW.md) describes interfaces and model boundaries.

## Measurements

| Panel | Baseline | Reviewed L1 | Interpretation |
|---|---:|---:|---|
| Selected composed gym, 30 positions | 24 optimal | 29 optimal | Five corrections; one check timed out. |
| Broader count-offer gallery, 117 positions | 100 optimal | 107 optimal | Seven corrections; 40 investigated choices retained; one timeout. |
| Development, 32 paired fresh deals | Reference | 0 wins / 0 losses / 32 ties | One changed move, no changed make/set outcome. |
| Final, 100 paired fresh deals | Reference | 0 wins / 0 losses / 100 ties | No changed moves. |
| Conditional hidden hands, 12 roots × 16 draws | Reference | 0 wins / 0 losses / 192 ties | No changed moves under the frozen decision realizations. |

Gym rows overlap; they are not 147 independent examples. Their action grades
are exact for the declared teacher model, not a measurement of population
strength. Mean make-probability regret fell from 0.8007 to 0.3492 percentage
points on the selected 30, and from 0.9832 to 0.7512 points on the broader 117.
All completed review values and changed grades matched the original answer
keys. Full finite root values were also tested against the separate finite
teacher implementation. Evidence: [30-position report](gym-30.json) and
[117-position report](gym-broad.json). Both original gym reports predate a
reporting-only revision to arena uncertainty output; their bytes remain
unchanged. A [fresh audit](report-reaudit.json) reproduced every result field
from the raw gym artifacts using the current reporter.

The final panel used seeds 99110000–99110099, identical deals/contracts across
arms, swapped partnerships, bid 30, ten concurrent games and two native threads
per game. It completed 200 games and 5,600 moves. Thirteen checks retained L1;
three were unresolved timeouts. The remaining reviewed primary decisions were
inactive. There were no original L1 fallbacks or exceeded overall deadlines.
The candidate and its limits were unchanged after the 32-deal development
panel (seeds 99010000–99010031).

All paired differences were zero. A descriptive paired bootstrap would
degenerate to [0, 0], so the report deliberately omits that interval. The
discordant-pair sign test has no discordant observations (p = 1). Neither
statement establishes equivalence or rules out gains in other positions.
Evidence: [development](development.json), [final](final.json).

### Cost and interruption

In the final live panel, mean elapsed decision time was **0.18262 seconds** for
the candidate and **0.18245 seconds** for L1, including forced decisions and
failed work. The separately measured review phases consumed 1.296 seconds over
2,800 candidate decisions: **0.463 ms per candidate decision** on average. This
is workload-specific; most moves do not trigger the extra calculation. The
small difference between concurrent mean latencies is not an isolated causal
timing estimate. The slowest recorded decision was 2.769 seconds.

Final pool execution took 110.13 seconds across a deliberate pause and resume.
At the pause, 11 pairs and 699 decisions were durable; all 699 saved decision
prefixes remained identical after completion. The 32-deal development panel
took 37.42 seconds. Watchdog allowances stayed below five minutes per slice.
See [execution receipts](run-receipts.json) and
[full-game resume verification](resume-verification.json).

### Same public position, different hidden hands

The conditional extension was declared after development and before final play.
It selected the first eligible baseline-arm coordinate from each of the first
12 qualifying final source deals, in seed/ply order. Selection did not inspect
value gaps, changed choices, or outcomes. Each root supplied 16 uniform draws
with replacement from its full mechanical hidden-hand support. Actual default
L1 opponents and deployed focal continuations then played both arms.

The 12 roots have supports ranging from 1 to 210 worlds. Repeated draws,
especially in tiny supports, are not distinct independent positions. The source
deal remains the grouping unit. This is a conditional experiment under a
uniform mechanical prior, not a behavioral posterior or prevalence estimate.

The runner saves the first realized decision for each exact own/public request
and complete player configuration. Across 192 paired worlds it computed 1,654
distinct decisions and used them 3,584 times. This measures outcomes of frozen
realizations of timed players; cached replay speed is not live-player latency.
All 384 complete replay records passed legality, request, and score audits.

An actual SIGINT interrupted the final version with 10 paired worlds complete.
All 154 durable files were byte-identical after resume to 192 complete pairs.
The initial operational version exposed a JSON tuple/list resume mismatch;
the runner was corrected and tested before this fresh interruption exercise.
Its original source and report identity are preserved separately, and its
duplicate outcomes do not increase the sample count. Evidence:
[conditional report](conditional.json), [resume verification](conditional-resume-verification.json),
[version provenance](conditional-provenance.json), and
[interruption harness](verify-interruption.py).

## Is the correction actually about partner?

After the evaluation, an attribution instrument compared every legal action on
the six originally missed selected positions. It held the complete finite
prior, lawful continuation search, and modeled opponents fixed, changing only
the modeled partner from L0-8 to L1-40/8. These diagnostic calculations were
allowed to complete rather than using the player's 250 ms cutoff.

Four of six missed offers are corrected even with the L0 partner. Two require
the different modeled partner to reverse the baseline-versus-offer preference:

| Selected case | Compared plays | L0-partner make mass | L1-partner make mass |
|---|---|---|---|
| advantage-22, 76 worlds | Baseline 4–0 / offer 6–4 | 54 / 52 | 52 / 54 |
| advantage-30, 120 worlds | Baseline 6–0 / offer 6–4 | 92 / 90 | 109 / 110 |

For advantage-22 a third play, 4–4, has L0-partner mass 55 and L1-partner mass
50; thus the all-action preference also reverses from a nonoffer to the offer.
These masses count make outcomes under their respective models over the same
worlds. They are not actual-deal outcomes or independent trials.

The attribution establishes partner-model sensitivity inside this checker.
It does not isolate every cause of original L1 error: the checker also replaces
sampled evaluation, has its own fixed field seed schedules, and optimizes lawful
future focal choices. Its field differs from live L1 opponents, and those
future optimum choices differ from the deployed continuation procedure.
See [full attribution values and identities](attribution.json),
[runner](attribute.py), and the `partner_review_ablation` native instrument.

## What survives, and the next question

The layered mechanism works operationally: a cheap public question can direct
a bounded, information-respecting investigation, preserve the baseline on
refusal, and occasionally improve a model-relative action grade. Counting every
correction as new partnership understanding would overstate the evidence.

The immediate-count detector produces too few useful changes in these fresh
games to establish a strength gain. Spending more whole games on the same
candidate is not the most informative next step. A stronger next diagnostic is
to find decisions whose ranking depends on partner behavior, including plays
that create a later opportunity rather than merely deliver count to a partner
already winning. Compare candidate explanations under matched partner models,
then test whether an affordable intervention changes outcomes with deployed
continuations. This is a proposed next cycle, not a completed finding.

## Verification and retained evidence

- 69 Python tests passed, including public gating, response refusal, config
  identity, durable decision caching, and JSON round-trip resume.
- 35 focused Rust tests passed across partner review, policy search, relational
  runtime/learning, learning I/O, and information prices. Strict clippy passed
  for the library, both new native instruments, and partner-review tests.
- Independent Python rules replayed 264 development/final arena games plus
  384 conditional records: **648 full-game replays**. Gym grades were checked
  separately. Replays and overlapping cases are not independent strength data.
- Campaign manifests pin sources, binaries, options, seeds, and field identity;
  checked-in reports retain requests, grades, audit hashes and execution
  receipts. Large raw decision/game files remain under
  `/Users/jason/data/texas-42/sunshine-count-*` as named in those receipts.
- The POLICY-ANTS subagent found the artifact names but could not retrieve the
  original contents. Only the [pending intake record](../../packet/policy-ants/INTAKE-STATUS.md)
  is preserved. No ideas or quotations are attributed to an unread document.

Full legacy CI remains under the existing session waiver. These are exploratory
implementation and experimental checks, not Lean proofs or a general guarantee
of stronger play.
