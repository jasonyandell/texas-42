# Unified player foundation

2026-09-06. EXPLORATORY engineering and matched-play evaluation.

## Authorized scope

Establish native L1 against the preserved phone, share the decision policy with
modeled L1 partners, then measure partner modeling and void handling on matched
deals. Build a reusable asynchronous, interruptible two-player arena. All
experiments use bid 30 and external foreground caps below 300 seconds. No phone
deployment, merge, or theorem claim is part of this work.

## Design obligations

1. One move-selection authority for fixed bundles, exact-tie refinement, and
   block race followed by refinement. Modeling level, belief strategy, selection
   rule, sample counts, and execution budget are distinct coordinates.
2. Completed action comparisons only. A deadline/refusal cannot turn a partial
   vector into a successful primary choice. Outer fallback is explicit.
3. Nested policies are pure in their own information and frozen configuration.
   They never see the host's hidden hands. A deadline aborts their computation;
   it does not choose a clock-dependent modeled move. Configuration is fixed
   before a shared policy cache is used.
4. All candidates in a comparison see common sampled worlds. Refinement uses
   fresh bundles, with the historical replacement rule. Racing preserves the
   historical paired-block estimator and elimination rule; it is exploratory,
   not a simultaneous statistical guarantee.
5. Legacy defaults retain behavior. L0 stays the existing fixed-sample response
   to Dice; selectable L1 rules apply to the real root and modeled levels >=1.
   The root and modeled budgets are independent and visible.
6. Two physical replays per deal, policy teams swapped, bid/trump/bidder fixed.
   Pair score is make(A)-make(B). Both-set and both-make pairs tie. Completed
   pairs/groups are published in planned order, never selected by runtime.
7. Durable checkpoints contain complete validated moves. Failed processes may
   retry; completed outcomes and valid deadline fallbacks are never rerolled.
   One shared pool owns the Mac budget. All player/panel/source identities are
   pinned before execution and checked on resume.

## Validation plan

Preserve existing fixed-policy fixtures; compare native race/refine decisions
against archived phone on matched information states; exercise shared root and
nested selection, both objectives, forced moves, deadlines, cache isolation,
and public-void support. Verify rule replay independently. Inject scheduling,
restart, and checkpoint corruption failures. Use short throughput checks before
fresh paired campaigns. Results will separate model strength from latency and
fallbacks, and fixed-hand groups from their repeated hidden completions.

## Implementation and checked obligations

`solver/selection.rs` owns the selection schedules. The existing L1 entry
points, the partnership root, and `Solver::pi` call that authority. A modeled
level k>0 descends to k-1; L0 bottoms out in Dice. Every play removes a tile.
The node solver still groups indistinguishable worlds before maximizing over
focal actions. No per-world clairvoyant maximization was introduced.

A nested mind reconstructs samples from its own hand/public key through the
selected belief strategy. The host's world vector is absent from this call.
Its frozen rule belongs to the shared evaluation/cache scope, with level and
public void context distinguished in keys. An incomplete nested solve aborts
its host; only the external player wrapper may choose a labeled fallback.

These are code contracts with focused witnesses, not a new formal proof of
Texas 42 mechanics or playing strength. The legacy rejection sampler still
has deadline checks in the bounded root; the parent enforces operational
termination. The sampled worlds are not the full physical fiber, and racing's
sign test is not a simultaneous validity guarantee.

Completed gates before the fresh battery:

- 44 focused Rust tests passed, with one pre-existing ignored sigma-repair
  probe. Selection schedule witnesses include exact 40/160/640 work, both
  objectives, saturation, a formerly lower candidate becoming best, refusal,
  and nested inheritance with immutable-cache reuse.
- Nine pre-change fixed-policy comparisons retain every exact value and move.
- 64 native/archived-phone comparisons agree at matched information states:
  32 race/refine choices and 32 full-refinement choices, including all available
  phone option values. The parity record pins binary hashes.
- 14 Python tests passed: score symmetry, independent player configurations,
  fixed-hand grouping, source identity, corrupted checkpoint refusal, worker
  timeout/restart, scheduling, and durable retry accounting.
- Native clippy passed with warnings denied; all workspace targets compiled;
  both browser crates compiled for wasm32-unknown-unknown. Full Rust CI remains
  waived under the launch scope.
- Ten active games were paused and resumed. All ten saved moves survived;
  all twelve completed pilot hands passed independent replay. Pilot results
  are development data, not additional fresh evidence.

Result derivation has one authority over validated moves. Commit-time replay
rejects a result that disagrees with its checkpoint. The parallel runner has
no lockstep game barrier and retains one native/phone worker per active game,
restarting failed workers under the parent deadline. Every request constructs
fresh native evaluator state and supplies explicit phone randomness.
