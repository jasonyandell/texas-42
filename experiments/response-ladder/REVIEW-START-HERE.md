# Walt fast core: review and research handoff

**Current result:** a lawful imperfect-information response core with cheap
compiled lower policies runs substantially faster, but the current compiled
player loses to current Walt on fresh hands. This is an experimental research
PR. The user paused the campaign for budget reasons; publishing this PR does
not resume it or change the production player.

## What to read first

1. [Paused handoff](PAUSED-HANDOFF.md): final state, measured results, retained
   production gains, failures and unexecuted work.
2. [Compiled lower-rung design](docs/COMPILED-LOWER-RUNGS-DRAFT.md): the operator,
   existing Scheme/gym/teacher infrastructure and competing failure hypotheses.
3. [Math-first GPU design](docs/GPU-WALT-MATH-REDESIGN-DRAFT.md) and
   [adversarial math review](docs/MATH-LADDER-REDESIGN-REVIEW.md).
4. [Campaign history](COMPILED-PLAYER-STATUS.md): all development variants,
   including negative controls. Earlier entries describe earlier states.
5. [Fresh confirmation protocol](docs/COMPILED-V5-CONFIRMATION-R2-PROTOCOL.md)
   and the complete panel summaries linked below.

## The mathematical boundary

A problem contains an ordered finite bundle of hidden scenarios and weights,
a lawful own-hand/public-history root, a frozen modeled field and an objective.
The response core uses MAX only at the focal player's information states and
SUM over public observations. A focal choice is shared by all scenarios with
the same complete public history. Original scenario columns, including
weighted duplicates, remain identifiable. This is not per-world perfect-
information optimization followed by a vote.

A completed certificate concerns that sampled problem and frozen field. It
does not certify the true hidden-hand distribution, the quality of the modeled
opponents, whole-game strength, or equivalence to historical Walt. Interrupted
search retains completed information and explicitly distinguishes unpriced
reserves from proved bounds.

The compiled ladder deliberately changes the lower policies. T0 teaches best
responses against Dice; a frozen compiled C0 approximates its action-cost
vectors. T1 then teaches responses against all-C0, and compiled C1 approximates
that target. The online field uses C0 opponents and a C1 partner, with C1 as
an optional focal tail. Changing C0 requires new T1 labels. The compatible
root prior/reset and historical Voidless lower minds are distinct targets.
Cheap compiled actors are not byte-compatible substitutes for those minds.

## Latest frozen candidate

The v5 family separates declaring/defending roles and leading/following phases.
Each phase program has at most three ordered relational clauses from a fixed
sixteen-clause vocabulary. The grammar is intentionally small and interpretable.
Inputs remain own hand plus public information; role normalization is internal.

- [C0-v5](results/compiled-c0-v5/actor.json), trained with 32-world T0 labels.
- [C1-v5](results/compiled-c1-v5/actor.json), taught against frozen C0-v5 with
  eight-world T1 labels.
- Online: outer40, plans1, horizon7, work2,000,000, compiled tail enabled,
  20ms per-move investigation cap, CPU backend.
- Opponent: actual current Walt L2 Partner 40/8/2 Fixed/Fixed Voidless,
  review off, six threads and its normal 14,000ms move allowance.

The comparison uses actual elapsed time with unequal configured caps. The
candidate does not obtain an apparent strength gain by weakening Walt.

## Evidence that should drive the next discussion

Each fresh panel has 576 independent physical deals, stratified equally over
nine declarations and four bidder seats at bid 30. Each deal is played twice
with partnerships swapped; all 28 tiles are played. Wins/losses/ties below
refer to paired contract outcomes, not individual tricks.

| Frozen panel | Wins | Losses | Ties | Candidate mean / p95 | Walt mean / p95 |
| --- | ---: | ---: | ---: | --- | --- |
| [C](review/FRESH-CONFIRMATION-SUMMARY.json) | 63 | 83 | 430 | 42.69 / 80.98ms | 90.53 / 216.33ms |
| [D](review/FRESH-CONFIRMATION-SUMMARY.json) | 44 | 74 | 458 | 42.53 / 77.07ms | 90.67 / 218.56ms |

Combined: **107 wins / 157 losses / 888 ties over 2,304 complete games**.
Mean partnership time is 42.61ms versus 90.60ms, about 2.13x faster. These are
14-decision partnership times, not 28-decision selfplay times. Both panels
are negative, so the predeclared positive-in-each-panel quality gate fails.
The earlier development 8/5/59 result came from repeatedly reused deals.

The harness reports all replays passed, zero failed arms/fallbacks, and clean
identities in all 32 bounded cycles. The additional independent final audit
is staged in [paused-staging](paused-staging/2026-09-21/README.md), not executed.
Do not describe it as completed or infer stronger play from local teacher loss.

The [exact trace-union compute control](results/trace-union-compute-v1/REPORT.md)
matched CPU action vectors and extracted-policy repricing on its 36 fixtures.
At seven remaining tiles CPU averaged 32.86ms versus GPU plus fold 72.93ms;
CPU trie folding/extraction dominated. This is a negative trick-1 compute
result for that implementation, not a theorem against GPU approaches.

The separate production CPU optimizations already landed before this research
PR. Their 12-pair engineering panel had a 13.1x median speedup with matching
completed choices/exact values. They are not a new result of this PR and should
not be combined with the compiled player's 2.13x result.

## Code map

- [core.rs](src/core.rs), [model.rs](src/model.rs), [mechanics.rs](src/mechanics.rs):
  lawful sampled response, policy/certificate structures and physical rules.
- [compiled.rs](src/compiled.rs), [compiled_family.rs](src/compiled_family.rs):
  strict actors, fast action selection, role/phase families and Scheme export.
- [compiled_player.rs](src/compiled_player.rs): actual online response and tails.
- [compiled-teacher.rs](src/bin/compiled-teacher.rs),
  [fit_compiled.py](tools/fit_compiled.py): finite teaching targets and exact
  weighted grammar selection. Historical-H0 diagnostic modes exist; their
  lesson campaigns have not run.
- [gpu_epochs.rs](src/gpu_epochs.rs), [epochs.wgsl](src/epochs.wgsl): compiled
  fields and continuation lanes on GPU, with CPU/GPU contract checks.
- [trace_union.rs](src/trace_union.rs): exact union of complete priority traces,
  full-public-history trie and adaptive SUM/MAX folding.
- [tests](tests): information-set, reference Scheme, legacy, tail, cancellation,
  CPU/GPU trace, weighted-column and player contracts.

Existing v5 validation includes 67 release contracts, seven fitter tests and a
WASM library build check. No experiment or training run was restarted for PR
publication. The native GPU harness uses Metal through wgpu; no browser/phone
performance validation of this compiled player has been done.

## Questions for the next reviewer

Start by checking the mathematical contracts and counterexamples, then build
a failure map. Distinguish at least these hypotheses:

1. **Representation:** the tiny ordered-clause grammar cannot express enough
   useful role/order/partner behavior, despite low local teacher cost.
2. **Training distribution:** real observation rows fail to cover the lower
   modeled queries that the online policy actually reaches.
3. **Target mismatch:** compatible/root-reset teachers approximate a different
   field from historical Walt's lower minds. Better imitation alone does not
   establish that either target is strategically preferable.
4. **Planning and finite sampling:** the online controller's finite bundle,
   search bounds and tail interact with policy errors. More outer samples
   already failed one development control, so sampling is not a free remedy.

Canonical action agreement, optimal-set agreement and normalized teacher loss
are separate diagnostics. Tied actions can change later public observations.
A proposed modeled-query census must capture actual FieldQuery own hands and
full public histories; do not substitute the referee's actual hidden hand for
a counterfactual modeled seat. Reconstruct a query's original own hand from its
remaining hand plus that seat's public past plays. Keep training groups intact.

A further C2 rung is a legitimate finite response experiment, but strength is
not monotone and programs can repeat/cycle. Identify its parent field/rung
explicitly rather than relabeling a C1 teacher invocation. No C2 has been run.

The useful next deliverable is one falsifiable diagnostic with a small explicit
budget and a stopping rule, chosen from the evidence above. A universal compact
quotient, stronger equilibrium player, or phone speedup has not been established.
Fresh C/D losses can inform subsequent development, but any later strength
claim requires new untouched deals.

## Review export and reproducibility

This PR contains all tracked source, tests, design documents, the immutable
reference source archive, frozen actors, and selected result/protocol/validation
files. To keep the diff focused on the implementation, selected result files
are packed byte-for-byte in `review/SELECTED-RESULTS.tar.gz`; the latest actors,
compact fresh-panel summary and trace-union report are also directly readable.
The original campaign contains 36,965 tracked files and roughly 6 GB of
raw results. The complete package is now public in the
[Hugging Face dataset](https://huggingface.co/datasets/jasonyandell/texas-42-walt-response-ladder),
including bulk game receipts, lesson corpora, request streams and cycle traces
from commit `25af9d07da66fe93229ada1f6122f2e6f08d34a9`. Its 49 raw archives
preserve every original byte, compressed to about 207 MB. The dataset also
provides browsable Parquet views of 4,542 game receipts (one partial), 127,170
observed moves, 15,862 canonical lessons, and the complete file inventory.
[Public dataset receipt](review/PUBLIC-DATASET.json) pins the verified Hub
revision, source commit, counts and publication checks.

[Export manifest](review/EXPORT-MANIFEST.json) records the included paths and
SHA256 digests and loose/archive locations. From this package,
`tar -xzf review/SELECTED-RESULTS.tar.gz -C .` restores the selected result files.
That archive does not contain the omitted bulk evidence. The compressed full inventory beside it records every original
git blob ID, size and path. Included original files are byte-identical copies.
The selected summaries support inspection; data-dependent replay and audits
also require the public raw corpus. Download instructions, checksums and an
export-integrity report are in the dataset card. Extract the desired campaign
archives into this package, preserving their `results/<campaign>/` paths.
Original source documents remain unchanged. Old manifests retain original
local paths and hashes; do not treat them as portable resume commands.

The Parquet decision table contains observed moves, not counterfactual modeled
FieldQuery samples. Preserve lesson physical groups/splits and campaign roles.
The shared `physical_deal_id` in games and decisions detects exact seat-preserving
deal reuse across campaigns; repeatedly reused development hands are not
independent strength evidence. Full hidden hands are for offline analysis.
Export checks verified all 36,965 archive members against original SHA256
digests and reproduced the recorded C/D outcome counts from the tables. This
publication check does not replace the unexecuted independent scientific audit.

For a fresh build, first run `python3 tools/prepare_reference.py` from this
package; it verifies and extracts the included immutable reference. Build/test
commands and worker wire format are in [README.md](README.md). Data-dependent
Python audits require the downloaded raw corpus. Staged audit files are
preserved for later review and are not part of the validated player runtime.
