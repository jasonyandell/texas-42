# Human-called Nel-O: implementation investigation

2026-09-20 · **Exploratory engineering evidence and proposed design.**
Tracking: [texas-42 #89](https://github.com/jasonyandell/texas-42/issues/89),
transferred intact from Plunge #3. No player implementation or deployment in
this investigation.

## Finding

The existing architecture can host this feature, but the shared player needs
explicit Nel-O mechanics and an objective before the table enables it.
Plunge already implements the selected house rules, including open auction
availability. Walt currently specializes both its ordinary and optimized
search paths to four-player straight 42. The work spans this repository and
Plunge; texas-42 is the appropriate home for the parent issue.

The first release remains human-called Nel-O with Walt defenders. Computer
Nel-O bidding/pricing is deferred. Simulations must nevertheless model a
declarer trying to avoid every trick.

## Source identity

- texas-42 `main`: `d24eaefe41a53dd625453253e021251281939d19`, verified against
  GitHub. The original workspace is on `walt-gran` at `9d6a5a2e`; the audit
  uses an isolated worktree from current main.
- Plunge `main`: `8bc71ca6e8dc2775f5cc9e559c2b22cccd1c8755`, also verified
  against GitHub.
- Plunge's checked-in WASM identifies texas-42 source `1dfd0e22f5fe2a2ee266f166493442547ad33834`.
  Its asset hash was verified before executing the boundary probes. The
  audited rules, solver, policy-search and walt-player source have no code
  difference between that source commit and audited main; the player README
  changed. Full manifest and hashes are in the receipt below.
- Rule references: [Plunge rules §8.1](https://github.com/jasonyandell/plunge/blob/8bc71ca6e8dc2775f5cc9e559c2b22cccd1c8755/docs/RULES.md#81-nel-o-nello--low--low-boy--no-trick),
  [its suit algebra](https://github.com/jasonyandell/plunge/blob/8bc71ca6e8dc2775f5cc9e559c2b22cccd1c8755/docs/SUIT_ALGEBRA_PURE.md),
  and the issue's confirmed rules. The existing [formal Straight 42 profile](../wiki/rules-profile.md)
  excludes Nel-O. Its theorem and receipt claims do not automatically extend.

## What already works, and what blocks release

| Surface | Audited behavior and required work |
|---|---|
| Plunge game rules | `src/engine/game.ts:180,210,274,337,429,462`: unpowered doubles suit, open Nel-O after a legal marks bid, skip partner, three-play tricks, immediate set, seven-trick make. Keep these semantics as the independent comparison. |
| Human auction | `src/engine/bidding.ts` already supplies the marks ladder. An ordinary marks winner can declare Nel-O when `nello:'open'`; the special Nel-O bid token is for the separate forced-bid route. `PLUNGE_CONFIG` in `src/engine/types.ts:93` currently inherits Nel-O off. `src/ui/sheets.tsx` already renders the declaration. Keep the nine-declaration computer auction and empirical book unchanged. |
| Shared rules | [`Decl`](walt/src/rules/decl.rs) contains only nine straight declarations. [`rules.rs`](walt/src/rules/rules.rs) makes every called tile powered and `Trick` contains four dominoes. Doubles-trump and ordinary no-trump both give the wrong Nel-O mechanics. |
| Public request and replay | [`walt-player::Request`](walt-player/src/lib.rs:31) accepts the existing seven fields and a 30–42 points target. [`partnership_wire`](walt/src/solver/partnership_wire.rs:56) rejects other declarations, derives actors modulo four, and reduces every seat's hand after each trick. [`solver::replay`](walt/src/solver/mod.rs:1802) shares those assumptions. |
| Objective and simulated play | [`Shared`](walt/src/solver/mod.rs:254), `child_after_play`, `solve_count`, and `hand_sizes_at` in the same file assume four-player tricks and the 42-point make/set complement. A reversed score or a synthetic bid value does not supply Nel-O termination. Every modeled actor must use the new objective. |
| Hidden deals | `sample_belief` already takes individual `[usize;4]` capacities. [`FiberDp`](walt/src/kernel/fiber.rs) and [`Kernel`](walt/src/kernel/kernel.rs) already represent three hidden seats with separate capacities. This is reusable machinery; the history-to-capacity constructors and declaration-dependent constraints need extending and checking. |
| Optimized CPU/WASM paths | The deployed `cpu-speedups` bundle uses [`compact_dice`](walt/src/solver/compact_dice.rs), its `const_objective` and `singleton` paths, and trick tables. These duplicate four-turn and 42-point assumptions. `Shared::normalized_boundary` uses `H + played/4` for cache transfer. Extending only the ordinary solver leaves the shipped path wrong. |
| Partner review | [`policy_search::request`](walt/src/policy_search/request.rs) and [`partner_rollout`](walt/src/policy_search/partner_rollout.rs) assume straight contracts. The current count-offer trigger is not a Nel-O defense policy. Initially route Nel-O through its own supported baseline and report the straight partner review as inapplicable, unless a separately tested Nel-O review is implemented. |
| Hosts and receipts | Plunge `src/ai/walt/requests.ts:140`, `src/ai/native.ts:56`, and `src/ai/review-request.ts` reject Nel-O. Add the contract/rule identity to requests, cache identities and receipt validation while retaining own/public inputs, checkpoints, cancellation and complete-result fallbacks. |
| Questions and inspection | Four-play indexing occurs in `src/questions/model.ts:49`, `src/ui/observation-link.ts:27`, `src/ui/Questions.tsx`, `src/ui/Table.tsx:124,285`, and `src/ui/NativeReview.tsx`. Replace flat-index conversion with one helper based on actual trick lengths, including the partial trick. Physical seat arithmetic modulo four is still correct and should not be globally replaced. |
| Replay/import | Plunge's engine-driven replay already handles three-player casual Nel-O. The combined forced-30/open-Nel-O configuration needs a stable encoded rule identity. The independent [`plunge_io.py`](../experiments/partnership/plunge_io.py), [`rules.py`](../experiments/partnership/rules.py), player normalization and gym consumers currently support straight contracts only. |

Plunge paths above are relative to its audited checkout; texas-42 links are
relative to this file. These are source findings, not a claim that every
research producer has been audited.

## Executed evidence

The existing Plunge suites `engine-rules`, `forced-bidding`, `walt-requests`,
`share`, and `questions` passed: **71 tests, five files**. These establish the
current baseline, not new Walt support.

The reproducible [probe](probes/nello-investigation-2026-09-20/probe.mjs) and
[JSON receipt](probes/nello-investigation-2026-09-20/results.json) add:

- Two deterministic legal traces, each rotated across all four declarer
  seats: **four first-trick sets and four seven-trick makes**. These are eight
  fixtures but only two distinct unrotated hands, not eight independent samples.
- **96 partial/final replay round trips and 96 observation checks**: correct
  trick shapes, identical replayed state, seven untouched inactive tiles,
  and hidden-pool cardinality equal to the hidden seat capacities.
- Four targeted doubles-suit checks: following doubles versus pips, ordering
  within doubles, and the absence of trump power on a pip lead.
- Three rejected requests through the **checked-in deployed WASM**, hosted in
  Node: an added Nel-O contract field, unused declaration ID 8, and a target
  of 1. They confirm that no existing wire encoding enables this contract.
- All eight fixtures are rejected by both the current question examiner and
  the independent Python importer at their straight-contract boundaries.

The make fixture is especially useful: the active players finish seven tricks
with **37 displayed points**, while the sitting-out hand retains five count
points. The contract is made. Any proposed Nel-O termination based on collecting
42 points fails this fixture.

The probe also reproduces a configuration trap: encoding
`{...PLUNGE_CONFIG, nello:'open'}` currently selects the casual replay preset,
whose decoder uses `allPass:'reshake'` instead of `force-30`. A new explicit
preset/version must preserve the intended auction rules and the meaning of
old links; changing a shared constant alone is insufficient.

Reproduce, using Plunge's installed esbuild dependency and Python 3:

```sh
node walt/probes/nello-investigation-2026-09-20/probe.mjs /path/to/plunge /tmp/nello-audit
```

The output directory also contains the disposable bundled Plunge probe. The
tracked receipt freezes the audited versions. This audit did not measure
defender strength, native/browser Nel-O decision parity, or phone latency.

## Proposed implementation contract

1. **Represent the contract explicitly.** Keep straight targets and the
   straight auction's nine choices stable. Add the selected Nel-O profile:
   doubles called into their own suit, no powered tiles, declarer identified
   by seat, partner inactive. Derive active seats from the contract, and reject
   inconsistent externally supplied combinations. Version request/receipt
   semantics so they cannot alias a straight decision.
2. **Keep four physical hands and three active actors.** After `t` completed
   tricks, an active seat has `7-t` tiles minus any current-trick play; the
   inactive seat always has seven. Its tiles stay hidden and eligible in
   completion sampling. It makes no move and supplies no failure-to-follow
   evidence. Use a common active-turn operation for replay and every recurrence.
3. **Define the terminal event directly.** Declarer success is zero declarer
   wins through all seven tricks; any declarer win immediately fails it.
   Declarer policies maximize its modeled probability, defenders minimize it.
   Preserve displayed trick points separately. Each simulated actor receives
   its own hand and public history, never the examiner's complete deal.
4. **Make the shared optimized paths obey the same contract.** Initially gate
   straight-only shortcuts away from Nel-O or generalize and check them.
   Include rules, objective, declarer/active seats, and capacity boundary in
   cache context/reuse guards. Keep deadlines and complete checkpoints in both
   native and WASM hosts. Do not promise that fewer actors alone meets the
   phone budget: the seven hidden inactive tiles still belong to the deal.
5. **Connect the table after the player is testable.** Enable open human Nel-O
   on the normal mark ladder, retain forced-30 handling, preserve existing
   saved-hand rules, and update replay presets, receipts, questions, inspection,
   hints and the independent importer together.

## Release gates

- Compare all ordered distinct three-tile trick winners against Plunge's
  independent rules, plus legal following, all declarer/leader seats and
  mid-trick states. Retain the pinned make/set traces above.
- Check capacity conservation, inactive-hand preservation, observed voids,
  and own/public policy inputs at outer and modeled-inner decisions. Enumerate
  small completion fixtures to check sampling against known support.
- Compare ordinary and enabled optimized recurrences, then native and WASM
  completed decisions. Test refusal after settlement, cancellation, expired
  budgets and complete-result retention. Cover the existing straight vectors
  and auction book as regressions.
- Run a reproducible defense bench with both declarer and defender policies,
  matched deals/seeds and fresh evaluation hands. Report early sets, makes,
  fallback frequency and thinking cost; preserve failures. Mechanics witnesses
  and modeled probabilities are not a defender-quality measurement.
- Exercise resumed auctions/hands, partial-trick questions, original receipts,
  final replay/observation links and gym import. Measure the actual Pixel/browser
  build before deployment; retain iPhone/browser portability as a constraint.

Recommended next slice: the explicit contract, unpowered doubles rules,
active-turn/capacity machinery, and independent mechanics fixtures. Then add
Nel-O L1 decisions and their bounded host behavior before enabling the table.
