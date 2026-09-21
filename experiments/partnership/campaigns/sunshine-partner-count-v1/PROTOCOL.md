# First sunshine experiment: investigate a missed count offer

2026-09-13. Exploratory. This protocol is written before new campaign outcomes.
It follows [Sunshine Notes](../../SUNSHINE-NOTES.md). POLICY-ANTS is optional
research intake and does not set this experiment's objective.

## Claim and scope

Default L1 sometimes misranks a legal count play that leaves its currently
winning partner ahead. The existing composed gym contains six such missed
choices among its 30 selected examples, including two ties in L1's estimated
values. This is a concrete failure family, not its prevalence in ordinary play.

Hypothesis: an optional bounded investigation of these offers can recover useful
partnership decisions at low average time cost. An opportunity alone is not a
reason to change the move; preserving the tile may be better.

## Intervention

`l1-partner-count-review` keeps default fixed L1 (40/8, legacy inner belief).
After a completed L1 choice, a public count-offer detector may request a check.
The detector specializes the maintained `offer-count.scheme`; the native check
evaluates that expression and must agree before a result is accepted.

Initial scope: bid 30, declaring-side actor, at most three own dominoes, at least
two legal moves, contract unresolved, and an available count offer not already
selected by L1. This is one immediate partnership opportunity, not general
partnership mastery. Opening decisions and defending-side decisions retain L1.

The check compares only the baseline action and query-returned offers. Each gets
unrestricted lawful focal continuation under the existing deterministic
L1-40/8-partner, L0-8-opponent GymField and the whole uniform mechanical root
fiber (cap 400). It retains original-world identities while conditioning on
future observed field actions. This is exact under that declared model; the
live arena's players and seed schedule differ. The actual future focal player
also continues to use the deployed procedure, not the check's extracted optimum.

Strictly better comparison mass may replace L1's action. Ties keep L1. Maximum
extra wall allowance is 250 ms, within the existing overall 14-second move
deadline. The node cap is 100,000. Timeout, excessive support, malformed output,
or unfinished comparison retains the completed L1. The existing original
fallback remains available if L1 itself fails. No count bonus is added.

## Discovery and development

First use the existing 30 composed exercises and the broader count-offer
gallery, including unfavorable offers. Retain exact teacher grades, detector
parity, changed/retained/unresolved outcomes, and time. These are development
diagnostics, not new held-out evidence.

Then use 32 fresh paired random deals, seeds 99010000–99010031, for development
and operational review. Fix any implementation defects before final evaluation;
record any changes to the policy or limits. Up to ten simultaneous games, with
two native threads per game, using the existing resumable pool. Every workload
slice uses the supplied watchdog and stays below five minutes.

## Held-out comparison

After development, freeze source and binary identities and play 100 fresh paired
deals, seeds 99110000–99110099. Same deal/contract in both arms, partnerships
swapped, bid 30. The baseline is unchanged default L1. No outcome-based optional
stopping or policy revision during this final panel. Technical failures may
pause the run; completed decisions remain durable and are verified on resume.

Primary endpoint: paired make(A) minus make(B), with both-make and both-set tied.
Report paired wins/losses/ties and uncertainty at the source-deal level. Secondary
endpoints: changed decisions, investigations attempted/completed/unresolved,
latency including failed checks, and total pool wall. Within-game observations
and compatible worlds do not count as independent deals.

Independently replay saved games to verify legality, scores, completion, and
review decisions. Compare the candidate with its saved baseline at reviewed
positions; distinguish field-relative root grades from actual completed games.
A small number of changes or discordant pairs may leave strength unresolved.

### Conditional hidden-hand experiment (declared after development, before final play)

Development exposed only one changed move in 32 deals. Keep the planned 100-deal
final panel, and separately concentrate measurement on relevant positions:
select the first eligible count-offer investigation from each of the first 12
qualifying final-panel source deals, using the unmodified baseline arm, seed/ply
order, and the same public gate and 400-world cap. Do not select by value gap,
changed choice, or outcome. If fewer than 12 qualify, use all that do.

For each selected public position, draw 16 independent uniform compatible
hidden completions with replacement using a separately pinned seed. Starting
from the same public history and each same complete deal, play out both the
candidate declaring team and baseline declaring team against actual default L1
opponents. This tests deployed continuations under a declared mechanical-root
prior, without needing the fixed GymField to predict the live players exactly.
It is a conditional experiment on detector-positive endgames, not a whole-game
or population strength estimate. Source deals are the grouping unit; sampled
worlds remain nested within them.

Persist and reuse each first realized public-information decision separately
for each player configuration. This ensures identical information receives the
same recorded decision across hidden completions, including any budget refusal.
This frozen realization of the deployed procedures is for outcome comparison;
the live full-game panel remains the latency measurement. Record actual computed
decisions, cached reuse, paired outcomes, and all complete game traces. Jobs are
independently resumable and use the same ten-worker ceiling and watchdog.

## What would be learned

Correcting selected gym errors validates the instrument on that field, not a
general strength gain. Failure on fresh deals may reflect field mismatch,
different deployed continuations, sparse opportunities, or insufficiently useful
investigation. High refusal cost would argue for a cheaper check or narrower
scope. Keep the outcome even if this candidate is rejected.
