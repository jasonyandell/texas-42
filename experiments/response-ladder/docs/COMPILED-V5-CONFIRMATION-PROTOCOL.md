# Frozen v5 complete-game confirmation

Prepared after nomination on development data and before any fresh games.
The nominated candidate is C0-v5/C1-v5 with separate declaring/defending and
leading/following programs, running the CPU backend. The source and binary
are identical to the completed positive development panel. GPU conformance
has passed, but GPU performance and phone performance are separate claims.

## Population and size

Use exactly two independent panels, `compiled-v5-confirm-a-20260921` and
`compiled-v5-confirm-b-20260921`. Each contains 16 independent physical deals
in each of 9 declaration by 4 bidder-seat cells: 576 mirrored pairs, 1,152
games per panel. The combined sample is 1,152 independent pairs / 2,304 games.
Contracts are fixed at bid 30 and assigned independently of the hands. Every
game plays all 28 tiles, including any contract-settled suffix. This tests
play after a contract, not bidding or strength at other bids.

Each pair retains the hands and contract while swapping the candidate and
current Walt between declaring and defending partnerships. Define
D = candidate-declaring make minus Walt-declaring make, taking -1, 0, +1.
Each of the 36 cells has equal weight. Independent namespace hashes determine
physical deals and policy seeds; a policy seed cannot reveal the actual deal.
Within a cell, alternate which partnership assignment runs first. The saved
freshness audit found no duplicated deals within or across the two panels,
and no collision with prior saved fixture panels.

## Frozen computation and measurement

Use the nominated C0/C1 actor files and compiled-player binary; exact identities
and fixture hashes live in `results/compiled-v5-confirmation/protocol.json`.
Configuration is outer40, plans1, horizon7, work2,000,000, compiled C1 focal
tail, and a 20ms per-move investigation cap. Current Walt remains its actual
production L2 Partner 40/8/2 Fixed/Fixed Voidless, review off, six threads,
and ordinary 14,000ms per-move allowance. These are unequal configured caps;
the comparison criterion concerns measured elapsed time at ordinary strength.

Measure elapsed request/response time for all 14 decisions of each partnership,
including forced and settled decisions. Report startup and off-panel warmup
separately. All per-request sampling, search and response handling is charged;
referee/checkpoint writing is shared harness overhead, not player latency.
Publish mean, median, p90, p95, p99, maximum and time by trick, with declaring
and defending latency separate as diagnostics. Compare actual mean and p95
partnership time in each full panel; both candidate quantities must be at
most 1.10 times current Walt. This operational tolerance is selected here
before fresh outcomes, not represented as a user-specified threshold.

Run one measured job at a time using the existing 55s cooperative cycles
under the 60s process-group watchdog. No competing build, fit, benchmark or
large Git operation runs during measurements. Preserve every completed move,
partial checkpoint, response, failure, fallback and cycle identity receipt.
A valid timed reserve remains scored. Never replay a failure, slow move or
losing outcome to improve a panel. Identity drift or an illegal/unreplayable
move is a technical failure; preserve the evidence and do not claim a pass.

## Stopping and interpretation

Complete both fixed panels regardless of the first panel's outcomes. Do not
change the candidate, seed namespaces, sample sizes, quality rule or latency
rule after fresh play starts. Do not teach on these hands during confirmation.
The final analysis must include all planned pairs and all missingness; any
technical failure or incomplete pair prevents confirmation under this run.
A failure or inconclusive result remains evidence and returns the work to
research. It does not authorize selecting a favorable subgroup or continuing
this same sample until significance. Any later candidate needs new evidence.

## Frozen statistical rule

The primary estimand is the equally weighted mean of the 72 panel-by-cell
means of D (16 independent physical pairs per stratum). It equals the pooled
mean here because the design is balanced. Require a positive mean in each
panel. For the pooled result let s_s^2 be the unbiased sample variance in
stratum s and Vhat = sum_s(s_s^2 / 16) / 72^2. Require the one-sided 99%
normal-approximation lower bound Delta - 2.3263478740408408 * sqrt(Vhat)
to be strictly positive. This is approximate large-sample inference for a
heterogeneous mean effect, not an exact finite-sample theorem. If variance
is zero, any stratum is missing, or any stratum size differs from 16, the
mean-effect inference is unresolved and the gate fails. Publish the variance,
standard error, one-sided approximate p-value, and two-sided 95% interval.

Also require the exact one-sided conditional sign probability
sum_{k=W}^{W+L} choose(W+L,k) / 2^(W+L) <= 0.01. This is a check against the
sharp null of within-cell exchangeable candidate/control labels, conditional
on discordance. It is not an exact test of the heterogeneous mean-effect
null. Independent sign permutations of equally weighted pairs would give
this same probability and are not an additional independent test. With no
discordances use p=1. Do not multiply the two probabilities or claim that the
two positive panels are independently significant.

Publish every cell's W/L/T, mean, variance and declaring-make/defending-set
rates, alongside both panel results. Include the conservative fixed-N 95%
Hoeffding interval for Delta, with half-width sqrt(2*log(2/0.05)/N), clipped
to [-1,1]. This interval has different assumptions/strength from the approximate
interval; it may remain wide and is descriptive, not another promotion gate.
For latency use nearest-rank quantiles: the sorted value at ceil(p*n), using
one-based indexing. No subgroup, individual declaration, bidding strength,
byte equivalence or universal superiority claim follows from a pass.

A pass requires both panel means positive, both pooled statistical gates,
both latency ratios at most 1.10 in each panel, and complete technically
valid execution. The claim is evidence of stronger average bid-30 play in
this declared uniform population at comparable measured time. No automatic
production deployment occurs in the harness.
