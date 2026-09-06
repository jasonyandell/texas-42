# Inner belief options in the unified solver

2026-09-06. **Exploratory engineering change.** No new strength campaign.

Jason asked for a clean option in the recently unified implementation rather
than an immediate fork. The shared solver now has an independent `InnerBelief`
strategy alongside its existing seat-level `Field`. Search, pmake scoring,
information grouping, recursion, Dice, and move ordering remain shared.

## Two strategies, one recursion

| Strategy | Modeled mind's support | Sampling | Default |
|---|---|---|---|
| `voidless` | own hand, played tiles, remaining capacities | historical shuffle stream | yes |
| `voids-counted` | those constraints plus public void deductions | existing `Kernel` / `FiberDp` / `sample_with` | opt-in |

The strategy is selected once on `Shared` before the evaluation is shared.
Every nested modeled level inherits it. Seat levels are a separate choice:
baseline, partner-only, and all-L1 fields work with either strategy. No new
sigma implementation or copied search was introduced. A small enum implements
the strategy boundary; an open trait registry is not needed for two known
implementations.

Existing entry points select `voidless` through the unchanged `Shared::new`
default. Existing sigma/field identities therefore keep their old meaning.
The partnership worker exposes the new selection explicitly. The preserved
phone WASM cannot select it and rejects that request in the wrapper.

The outer sampler stays the existing void-conditioned shuffle-and-reject
sampler. This keeps the outer worlds matched between strategies at equal
public inputs, seeds, and sample counts. The new inner sampler uses a different
deterministic stream; action changes cannot be attributed to void conditioning
alone without a further controlled sampling comparison.

## State and cache contract

Both `Key` (search memo) and `PiKey` (modeled-policy cache) include
`voids: Option<[u32; 4]>`:

- `None`: legacy approximation; later simulated voids are also ignored.
- `Some([0; 4])`: tracked opening state with no deductions yet.
- `Some(masks)`: tracked public forbidden-tile masks, indexed by internal seat.

The last two are different from `None`. Treating an empty tracked state as
legacy would lose every future failure-to-follow deduction.

`inner_belief::after_play` is the common void-update authority for searched
children and replayed continuations. It reads the pre-play leader and partial
trick, uses declaration-relative following, and preserves deductions when a
trick clears. The root starts from the complete public replay's existing
void masks. A solver refuses a key/strategy mismatch via an invariant assertion.

Cache scope is one evaluation with one immutable strategy. Different public
void profiles cannot share policy or search entries. Legacy callers initialize
the new coordinate to `None`, preserving their previous equivalence classes.
The old seed hash and shuffle consumption are unchanged for legacy evaluation.
The extra key storage may have a runtime/memory cost; trace parity is not a
claim of byte-identical cache layout or identical timing.

## Counted sampler adapter

The adapter converts replay's forbidden tile masks to the kernel's effective
contexts, asserting that the incidence union reconstructs the mask exactly.
It constructs a kernel from the modeled seat's own hand, public played tiles,
capacities, and voids. It has no access to the host's hypothetical worlds or
actual other hands. One completion-count table is reused for that mind's
requested samples. The existing kernel supplies integer-weighted sampling.

An inconsistent frame is an invariant failure, not permission to sample an
unconstrained replacement. Deadline expiration discards incomplete evaluation.
The wrapper's complete fallback uses the same selected belief strategy; it
may still fall back to a labeled legal move if evaluation does not finish.

This is uniform **physical support**, not an action-likelihood posterior and
not a fitted partner model. Existing information approximations beyond public
mechanical voids are unchanged.

## Use

All commands are run from the worktree root. Existing defaults are unchanged.

```sh
python3 experiments/partnership/player.py --mode partner --inner-belief voids-counted
python3 experiments/partnership/table.py --inner-belief voids-counted
```

`experiment.py` accepts the same flag for native seats. `campaign.py init`
accepts it and freezes it into the manifest and campaign ID. Phone seats stay
on the archived reference. Existing manifests without the field mean legacy;
source/binary identity checks still prohibit resuming a campaign with a changed
implementation. The shared pool consumes this frozen setting through the
existing campaign worker; it needs no second scheduler.

No campaign was created or launched for this change. The temporary manifest
test creates no games and deletes its temporary directory.

## Validation

- Nine pre-change native decisions across baseline, partner, and all-L1:
  every legacy action value and chosen tile reproduced exactly. Saved input,
  old binary digest, and rational values: `runs/voids-before/golden.json`.
- The same nine decisions complete under counted beliefs. Tiny settings
  (4 outer, n0=2, n1=2) are compatibility smoke checks, not playing-strength
  or production-latency evidence. Records: `runs/voids-compatibility/`.
- Eleven partnership Rust gates pass, including membership of new samples in
  independently replayed receipt fibers, repeated sampling identity, public
  deductions across trick completion under all nine declarations, cache
  separation, host-world independence at levels 0 and 1, all three field
  profiles, and expired sampling.
- Python integration verifies strategy selection for primary and fallback,
  invalid/phone options, manifest identity, and the existing 252 independent
  Python/native position comparisons and timeout/failure checks.
- The entire Rust workspace's targets compile. This is a compile check, not
  a full test-suite execution. Both browser crates also compile for the actual
  `wasm32-unknown-unknown` target (`runs/voids-wasm-check/`).
- Existing kernel sampler (5), sigma1 repair (7, plus one already ignored),
  policy (7), viewer-fiber (2), and ordering (4) gates pass: 36 focused Rust
  tests including partnership. Records: `runs/voids-regressions/` and
  `runs/voids-regressions-retry/`. The first direct test compilation lacked
  Cargo's manifest-directory environment variable; the retry supplies it.

The next experiment can compare the two native belief settings while holding
the partner-only field, bid 30, and paired deals fixed. No improvement claim
or production-default change is made by these checks.
