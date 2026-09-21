id: [[inner-voids-default]]
opened: 2026-09-13 (main-side counterpart of `walt/kanban/backlog/inner-voids.md` on the unmerged branch `walt-o5`, opened there 2026-09-04)

## What

Whether the modeled minds sample the void-conditioned fiber — obligation O5
(`walt/SCENARIO-PLAYER.md` §10). Measured on branch `walt-o5`: the
void-blind minds believe a fiber the record has already refuted — median
250–363‰ dead from trick 3, max 988‰; the σ0 flip rate GROWS with n0
(102‰ → 189‰ at n0 2 → 8), a bias floor, not variance; level-1
recommendations move 90–454‰ by trick; the mirrored match is
epoch-dependent after a seed repair (live epoch 33/19/20 pairs aware,
reduced epoch 76/83/33 blind; "dead heat" withdrawn).

Two implementations exist as of 2026-09-07 and neither is the default:

- (a) `walt-o5` (9 commits, UNMERGED): `Level0Field::void_aware` behind the
  `Key::voids` epoch marker, shuffle-and-reject with the carried mask;
  gates `walt/walt/tests/solver_inner_voids.rs`; record
  `walt/probes/o5/README.md` on that branch.
- (b) main: `walt/walt/src/solver/inner_belief.rs`
  `InnerBelief::VoidsCounted` (dbcc698f, the kernel's exact counted
  sampler); batteries 9/5/36, 5/8/37 (racing L1) and 12/17/71 (L2 Partner);
  the two settings use different inner draw streams, so void conditioning
  is not isolated (`experiments/partnership/INNER-BELIEF.md`).

Jason's ruled design for any default (2026-09-05): rank/unrank over a
counting DP — reuse `kernel/fiber.rs`'s `FiberDp` or the model-belief
capacity DP rather than shipping a second counter; uniformity structural
(`N` indices unrank to `N` distinct lawful deals, `rank(unrank(i)) == i` on
a pinned root); follow semantics through `Decl::follows` /
`effective_incidence`, never reimplemented. **The flag's default is Jason's
word, not a builder's.**

## Done when

One implementation is chosen (or the two reconciled) with the rank/unrank
sampler; a matched-draw-stream comparison isolates void conditioning; the
play case is made or refuted at a declared epoch with a pre-declared
stopping rule; the O5 row in `walt/SCENARIO-PLAYER.md` is rewritten once
(both branches currently carry a different version of it); Jason rules the
default.

## Links

`walt/SCENARIO-PLAYER.md` (O5 row, the 2026-09-06 inner-belief note),
`experiments/partnership/INNER-BELIEF.md`,
`walt/briefs/MORNING-2026-09-05.md` (item 5, call (D)),
`wiki/walt-gran-anchors.md` §8, [[partnership-strength-question]],
[[gran-anchor-reconstruction]].
