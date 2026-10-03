# Staged strict confirmation reporter

Files: `/tmp/compiled_confirmation_report.py` and
`/tmp/test_compiled_confirmation_report.py`. No live edits or actual panel
aggregation. Synthetic tests are staged but unrun: execution was deferred when
the measured lane started. Inputs are explicitly parameterized; R2 C/D is bound
by its root protocol and output_directories map. The excluded predecessor setup
ID is preserved as context, without reading its outcomes or adding its games.

After BOTH panels finish and the compute lane is available, run the synthetic
suite under the usual watchdog, then the reporter with:

```
python3 /tmp/compiled_confirmation_report.py \
  --spec /path/to/response-ladder/results/compiled-v5-confirmation-r2/protocol.json \
  --panels /path/to/panel-c /path/to/panel-d \
  --harness /path/to/response-ladder/tools/h2h_compiled.py \
  --output /path/to/new/final-report \
  --work /path/to/separate/validation-work --seconds 55
```

Wrap each invocation in the60-second process-group watchdog. Repeat the exact
command on a cooperative budget return; it validates pending whole games and
retains complete cache transactions. The report directory is created only at
a terminal valid/invalid result. No inferential result is emitted on a budget
return or a technical refusal. A technical failure is sticky and never silently
retried. This script does not run players or mutate campaign output directories.

The work manifest binds script, root spec, harness, both fixture hashes, all
panel metadata and outer-cycle receipt hashes. Each derived game checkpoint
contains the SHA256 of its source game and a SHA256 of its compact sufficient
statistics. Reuse rehashes the original receipt, verifies compact checksum,
fixture/arm and shape, all28 moves and14 per partnership (2 per trick). Every
new game is independently replayed through the existing rules/harness and its
full move legal sets, player roles, public requests and response choices are
checked. Only compact statistics are kept in memory after one game. Cached
validation is trusted local derived state with corruption checks, not an
adversarial proof; editing both compact data and its hash is outside that model.

A final cycle must reverify the hashes of all2304 games. End checks verify file
size/mtime has not changed since reading each game and recompute all panel,
outer-cycle and current source/binary identities. Inputs must stay frozen.
The reporter rejects missing, extra or renamed game receipts, leftovers,
failures/startup-errors, missing cycle ends, identity drift and wrapper failures.
R2 outer run.json receipts must exactly cover the inner cycles and bind the
nominated commands/configuration. Missing original setup cycles cannot leak
into the replacement analysis.

Fixture authority is the existing generator plus pinned hash: declarations are
0,1,2,3,4,5,6,7,9; indices are global0..575, not repeats0..15. Each panel has36
cells ×16 independent pairs; the pooled analysis has72 cells. Mean and unbiased
sample variances and the stratified variance estimator use Fraction. Normal
limits/p-values use the specified floating constants; exact sign tails are
integer binomial sums over2^(W+L). Zero pooled variance leaves mean inference
unresolved and fails its gate. Sign and approximate mean inference are labeled
with their different nulls. No probabilities are multiplied.

Both panels must have positive means; both pooled statistical gates and both
per-panel mean/p95 latency gates must pass. Latency quantiles are nearest rank,
including median. Decimal receipt times become exact Fractions for means,
partnership sums and the11/10 gates. All14 requests, including forced, settled
and valid reserves, count. Startup/warmups are separate. Per-role partnership
latency, request latency and per-trick distributions are published. Every
cell has W/L/T, mean, sample variance and declaring-make/defending-set rates.

Eight staged synthetic tests cover exact unequal-sign variance, sign tails,
zero variance/missing/unbalanced cells, opposing panel means, nearest-rank and
exact latency threshold, real fixture shape/global index575/declaration9,
independent physical replay and request mutations, hash-bound resumable cache
and corruption, missing/renamed game files and cycle failure/drift. They read
source APIs, not fresh outcome receipts. No executed-test claim yet.
