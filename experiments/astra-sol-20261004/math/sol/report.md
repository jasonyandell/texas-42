# Sol adversarial mathematical review

2026-10-04. **EXPLORATORY RESEARCH.** These abstract countermodels challenge
generic accelerator guarantees. They are not Texas 42 playing-strength results,
not exact game theorems, and do not promote Walt's evidentiary tier.

## Kernel-checked result: arbitrarily small inner error can cause full conditional outer loss

Let a modeled field mind see two hidden types, `true` and `false`, with exact
masses `n` and `n+1`. It wins by choosing the matching type. Its exact action
values are `(n+1)/(2n+1)` for `false` and `n/(2n+1)` for `true`; its unique best
action is `false`. An approximate table that exchanges these two values has
uniform action-value error exactly `1/(2n+1)`, chooses `true`, and incurs modeled
policy regret exactly `1/(2n+1)`.

The host has a different information set: it knows that the actual type is
`true`, and it succeeds when the field guesses incorrectly. Replacing the exact
field by the approximate field changes host success from 1 to 0. Both field
policies are constant and therefore lawful; neither reads the hidden type.
For every positive integer `m`, choosing `n=m` makes the inner error and regret
strictly smaller than `1/m`, while the conditional outer loss stays 1.

`ArgmaxAmplification.lean` proves this family using exact natural-number payoff
numerators and denominators. The ratio comparison is represented by its exact
cross multiplication `m < 2*n+1`; it does not invoke floating point or silently
assume a numerical division convention. Six printed theorem axiom lists contain
only standard `propext`. There is no `sorry`, `native_decide`, or custom axiom.
The core-only Lean v4.33.0-rc1 run completed successfully in 0.150 seconds;
`lean-run/run.json`, `stdout.log`, and empty `stderr.log` are its receipts.

**Consequence.** Inner teacher value error or inner-policy regret alone cannot
bound the upper mind's conditional value loss by a quantity tending to zero.
The lead's policy-disagreement/margin result supplies the missing hypothesis.
This does not contradict continuity of the optimal *value* under uniformly
perturbed values on one unchanged belief; it concerns the discontinuous chosen
policy and its effect under another seat's information.

An independent Python implementation uses `fractions.Fraction`, checks action
errors, policy regret, lawfulness, and the host loss at
`n ∈ {1,2,10,1000,1000000000}`. Its receipt is `witness-run/run.json`, with full
machine-readable tables in `witness-run/stdout.log` (0.043 seconds, exit 0).
The expanded final witness including the incumbent-only challenge also passes;
`witness-final-run/run.json` records 0.048 seconds and exit 0.

## Why memoization does not itself prove a useful linear cost in k

The executable witness also evaluates a binary demand tree with complete query
keys `(remaining level, complete branch path)`. Every key is distinct. Perfect
memoization avoids a repeated root query but still evaluates `2^k` leaves and
`2^(k+1)-1` total queries. At `k=12` this is 4096 distinct leaves and 8191 total
queries. It demonstrates the elementary obstruction to the claim that merging
identical complete keys alone turns arbitrary nested response demand into
linear work. These are abstract query counts, not Walt timing measurements.

The precise conditional positive statement is: if every rung needs at most `M`
distinct information queries, every query has local cost at most `C`, and
dependency scheduling/memoization add bounded overhead per dependency, then
work is bounded by approximately `(k+1)*M*C` plus dependency overhead. A fixed
finite game may supply a huge `M`, so this review does **not** claim that every
asymptotic linear-in-k bound is impossible. A useful bound must control `M`,
local solve cost, and the count of dependency edges for the actual demand
domain. A compiled constant-time policy has bounded query cost, but its online
exactness is against that compiled field; it does not make the teacher ladder
exact for free. A finite physical horizon also limits expansion depth.

## Higher k has no generic strength guarantee

The executable rock-paper-scissors witness iterates the unique stationary
best-response rule: rock → paper → scissors → rock. Against a fixed real rock
opponent, the first response wins and the next response loses. This is a useful
regression witness, not a new mathematical discovery: the existing review in
`experiments/response-ladder/docs/MATH-LADDER-REDESIGN-REVIEW.md:133` already
states it and cites `targeted_level2_field_stability_v0.1.md` L2-T5. That historical
review was read in `/Users/jason/.codex/worktrees/walt-response-ladder/texas-42`,
not assumed to be part of the isolated baseline.

Even eventual periodicity of a stationary finite best-response operator needs
care in Walt: `walt/walt/src/solver/mod.rs:1237` selects the rung's sample count
and `:1239` changes the seed with absolute rung index. Thus the implemented
sampled tower is not automatically iteration of one stationary operator.

## Implementation evidence and distinct meanings of exactness

All current-code locations below are relative to the isolated baseline at
`3cf2536dcbd6636bfff8ab814858ee00e8dc8b5e`.

- `walt/walt/src/solver/mod.rs:1203` states modeled policy purity in `(k,key)`;
  `:1214` constructs the modeled seat's own information key; `:1221` caches
  completed answers; `:1237` selects its finite sample budget. No actual other
  hidden hand becomes an observation merely because a world is sampled.
- `walt/walt/src/solver/partnership.rs:22` separates Baseline, PartnerOnly,
  and AllLevel1 fields. `:38` gives Baseline all modeled L0 minds; `:43` raises
  only the fixed partner. The focal seat alone optimizes, so PartnerOnly is not
  joint optimization of partners' private policies.
- `walt/walt-player/src/lib.rs:83` always requests `baseline` through the wire
  with fixed selection and `inner_belief 0`; `:125` labels this Voidless.
  `:142` begins the small 8/2 fallback; `:152` requires ordinary 40/8 before a
  request with more than 40 worlds; `:169` retains only completed comparisons.
  The optional partner rollout at `:191` is a separate review, not a nested
  PartnerOnly L2 exact solve. Current code therefore differs from old
  archived racing-player descriptions.
- `walt/walt/src/solver/partnership_wire.rs:163` maps `baseline` to the
  Baseline field profile. Reporting `n1=2` in this route does not imply that an
  L1 modeled partner was actually evaluated.
- `walt/walt/src/solver/focal_ladder.rs:8` instead describes an append-only
  focal-horizon bound ladder at one fixed root/field/tail identity. `:19`
  intersects completed intervals, `:42` shares lawful focal maxima, and `:55`
  derives monotone retained gap. This monotonicity is not a theorem about
  raising the field's model level or resampling its worlds.
- The separate response-ladder prototype in the main checkout's README and original
  `tests/core_contract.rs:511` exhibit a clairvoyant upper strictly above the
  lawful sampled response. Revealing a world/tape to future focal choices is an
  upper relaxation, not an executable strategy. Root certification, canonical
  root certification, optimal incumbent value, complete exact action vector,
  and off-sample playing strength are distinct outputs.

The lead owns actual current WASM/profile evaluation and deployed-byte tracing.
This report has not measured an iPhone, claimed a production rollout, or
transported abstract witnesses into exact Texas 42 facts. Source checkouts were
only read; all artifacts and bounded runs reside in the isolated research tree.

## Independent review of the lead's proof

The initial five theorems in `math/lead/Disagreement.lean` are sound as written.
The Boolean weighted payoff sandwich assumes matching payoffs whenever the
declared disagreement flag is false. It accepts repeated list entries, preserving
their scenario multiplicity. Strict and weak action transport have the necessary
one-sided value bounds. The canonical interval-pruning theorem correctly requires
strict exclusion for a smaller-index competitor and permits equality for a
larger-index competitor, under a maximizing objective.

An independent run of that file passed in 0.372 seconds, recorded in
`lead-independent-lean-run/run.json`. At the time of this review the proof does
not derive game coupling or supremum transport automatically; those remain
explicit premises or a further lemma. For T0 use the host-success objective or
complement T1 values with exchanged interval endpoints.

The critical falsification test is now executable: old root A has payoff vector
`[1,0]`, B `[0,0]`; under the changed field A stays `[1,0]` while B becomes
`[1,1]`. The incumbent A has zero trajectory disagreement, but the best root
changes to B. Thus a disagreement estimate from only the incumbent trajectory
cannot justify transporting all action optima. Safe transport needs action- and
policy-uniform bounds, or independently proved one-sided bounds at the maxima.
The final witness receipt includes this case.

## Production-byte and reusable-harness audit

The lead subsequently identified the supplied Plunge production commit
`a0d9fa806166b0e63fe016bb49d93f91f47b1af8`, obtained its exact player bytes,
and moved the isolated tree to source commit
`cb1ef3b23072e4c268f31f625f2b61d5facc1929`. The earlier code citations above
refer explicitly to the initial `3cf2536d` source; the reviewed straight
decision procedure has the same behavior. The local Plunge `aff966d6` snapshot
is stale and is not the production authority for the final harness.

An independent audit checked production WASM SHA-256
`40d7a1eea658627f7bab37d3c1888ad3bf00216fb23a5d2ceeb7d2d1f4219119` against
the copied production manifest. `harness_audit.py` then queried these exact bytes
for 144 forced suffix decisions: nine straight declarations, all four bidder
rotations, all four partial-trick offsets. It checked every legal set, choice,
leader, and score against the independent referee, completed and re-audited
each full game transcript, alternated the partner-review flag, and checked the
request construction's syntactic own/public information boundary. All 144 calls
returned the forced route with fresh WASM instances. The bounded run passed in
0.200 seconds (`harness-audit-run/run.json`). These are ABI/profile/referee
checks, not a nonforced-search or playing-strength experiment.

The copied production `native.ts` profile agrees with the arena: ordinary
native-partner requests use 40 worlds, the default 14-second budget, and review
enabled; the bidder's first move uses 160 worlds, 20 seconds, and review disabled.
The fixed public policy seed is independent of hidden deal generation. The
paired contrast `made(candidate declaring) - made(candidate defending)` has the
correct sign. Its whole-source-deal average lies in `[-1,1]`, and the stated
Hoeffding radius `sqrt(2*log(40)/N)` is correct conditional on independent source
deals and an unbiased prespecified campaign. Node's reusable compiled module
and host timings remain explicitly distinct from actual phone latency.

Two harness issues were reported to the lead for coordinated repair:

1. The copied production `client.ts` retains a saved checkpoint on worker
   `message.error` or `worker.onerror`. The reviewed arena version instead raises
   on a top-level error at `arena.py:79` or worker EOF at `:73`, even if it has a
   saved checkpoint. Timeout checkpoint retention is correct. To preserve the
   production error behavior, saved checkpoints should also survive those
   worker-error paths with an explicit interruption reason. A `result.error`
   should continue to fail, matching the production client's separate branch.
2. The reviewed summarizer reads completed game files only. It cannot discover
   a source seed with no completed games, and its descriptive pair counts include
   incomplete eight-game blocks while its confidence interval includes only
   full blocks. Expensive/failing games can be censored by the 240-second game
   cap, which is below the sum of all live per-move allowances. A reusable
   strength campaign needs a prespecified attempt manifest, failed-attempt
   records, and explicit completeness accounting. A pilot labeled
   `strength_claim=false` can remain descriptive, but must not hide exclusions.

No harness source was edited by this reviewer. The final six-theorem lead file,
including `maximum_transport`, was independently reviewed and rerun successfully
in 0.314 seconds (`lead-final-independent-lean-run/run.json`). The added lemma
correctly uses an attained old value, a target upper, and a uniform one-sided
error bound over the same policy domain. It can be applied symmetrically for a
two-sided maximum bound; it does not claim to formalize Texas 42 game coupling.

## Resolution and full final-panel replay

Both reported harness issues were repaired by the lead. The final arena retains
the saved checkpoint on top-level worker errors and EOF, annotates the
interruption, and keeps `result.error` fatal. The panel driver writes its full
plan before play and records each running/completed/failed attempt. The final
summarizer takes expected source seeds from that plan, counts paired outcomes
only for complete eight-game blocks, and refuses a population interval whenever
a planned pair is missing. Declaration is now a per-source stratum, with matched
declarations required inside each partnership-swap pair.

`panel_audit.py` independently replayed **all 32 production-panel games and all
896 decisions**, using the independent rules and reconstructing each payload
directly from the original actor hand and public transcript. It checked the
prespecified plan, all 32 successful attempts, exact shuffled/rotated deals,
all actor turns, legality sets, choices, scores, leaders, final scores, bid
outcomes, role/profile flags, policy seed, and both artifacts' hashes against
the production manifest. Recorded arena/adapter/referee hashes also matched.
The current detached source is `cb1ef3b23072e4c268f31f625f2b61d5facc1929`.

The independently recomputed totals agree: four complete source clusters,
16 paired ties, 448 decisions per side, and **6,368,000 ms requested budget per
side**. Phone routes are 285 baseline, 161 forced, and two baseline-reviewed;
candidate routes are 288 baseline and 160 forced. There were no fallback routes
or interruptions. The conservative 95% interval is `[-1,1]`; the panel remains
a descriptive smoke result and establishes no playing-strength equivalence or
advantage.

Three synthetic protocol tests exercised actual `Worker.call` behavior:
checkpoint followed by worker error retained the checkpoint; checkpoint followed
by EOF retained it; a final result containing an error remained fatal. Two
temporary-copy censor tests removed one planned receipt and added a wholly
absent planned source deal. Partial blocks were excluded from paired counts,
missing seeds stayed visible, and both cases refused an interval. The originals
were not edited.

The complete audit passed in **0.265 seconds** under the 30-second process-group
watchdog, with empty stderr and receipt `panel-audit-run/run.json`. Both earlier
harness caveats are resolved for this final panel. Remaining scope restrictions
are deliberate: play only, fixed bid 30, four source deals, deterministic public
policy seed, Node WASM host timing rather than an actual phone, and no claim that
the independent game mechanics have been formalized in Lean.
